#!/usr/bin/env python3
"""
Locus Factory - Full Pipeline Automation

Automates the complete problem generation workflow:
1. Fetch topics/subtopics from main backend
2. Generate scripts for all combinations
3. Mass generate problems
4. Export and upload to backend

Usage:
    python automate_pipeline.py [options]

Options:
    --skip-generation       Skip script generation if scripts already exist
    --problems-per-script N Override default 100 problems per script
    --topics "topic1,topic2" Only process specific topics
    --dry-run              Show what would be done without executing
    --clear-before         Clear staged problems before starting
    --main-backend URL     Main backend URL (default: http://localhost:3000)
    --factory-backend URL  Factory backend URL (default: http://localhost:9090)
    --grading-mode MODE    Grading mode (default: equivalent)
    --timeout N            LLM timeout in seconds (default: 300)
    --log-file PATH        Write detailed logs to file
"""

import asyncio
import httpx
import argparse
import sys
from datetime import datetime
from typing import List, Dict, Optional
from pathlib import Path


class PipelineConfig:
    """Configuration for the automation pipeline"""

    def __init__(self, args):
        self.main_backend = args.main_backend
        self.factory_backend = args.factory_backend
        self.problems_per_script = args.problems_per_script
        self.grading_mode = args.grading_mode
        self.timeout = args.timeout
        self.skip_generation = args.skip_generation
        self.topics_filter = set(args.topics.split(',')) if args.topics else None
        self.dry_run = args.dry_run
        self.clear_before = args.clear_before
        self.log_file = args.log_file
        self.overwrite = args.overwrite
        self.max_retries = 3
        self.retry_delay = 2.0  # seconds


class PipelineLogger:
    """Simple logger with console and optional file output"""

    def __init__(self, log_file: Optional[str] = None):
        self.log_file = Path(log_file) if log_file else None
        self.start_time = datetime.now()

    def log(self, message: str, level: str = "INFO"):
        timestamp = datetime.now().strftime("%H:%M:%S")
        formatted = f"[{timestamp}] {level}: {message}"
        print(formatted)

        if self.log_file:
            with open(self.log_file, 'a') as f:
                f.write(formatted + '\n')

    def info(self, message: str):
        self.log(message, "INFO")

    def success(self, message: str):
        self.log(message, "SUCCESS")

    def warning(self, message: str):
        self.log(message, "WARNING")

    def error(self, message: str):
        self.log(message, "ERROR")

    def section(self, title: str):
        separator = "=" * 60
        self.log(f"\n{separator}\n{title}\n{separator}", "")


class PipelineStats:
    """Track statistics during pipeline execution"""

    def __init__(self):
        self.topics_processed = 0
        self.scripts_generated = 0
        self.scripts_failed = 0
        self.problems_generated = 0
        self.problems_uploaded = 0
        self.failures: List[Dict] = []

    def add_failure(self, stage: str, item: str, error: str):
        self.failures.append({
            'stage': stage,
            'item': item,
            'error': error[:200]  # Truncate long errors
        })

    def summary(self) -> str:
        lines = [
            "\nPipeline Execution Summary:",
            f"  Topics processed: {self.topics_processed}",
            f"  Scripts generated: {self.scripts_generated}",
            f"  Scripts failed: {self.scripts_failed}",
            f"  Problems generated: {self.problems_generated}",
            f"  Problems uploaded: {self.problems_uploaded}",
        ]

        if self.failures:
            lines.append(f"\nFailures ({len(self.failures)}):")
            for f in self.failures[:10]:  # Show first 10
                lines.append(f"  [{f['stage']}] {f['item']}: {f['error']}")
            if len(self.failures) > 10:
                lines.append(f"  ... and {len(self.failures) - 10} more")

        return '\n'.join(lines)


async def retry_request(client: httpx.AsyncClient, method: str, url: str,
                       config: PipelineConfig, logger: PipelineLogger,
                       **kwargs) -> httpx.Response:
    """Execute HTTP request with retry logic"""

    for attempt in range(config.max_retries):
        try:
            response = await client.request(method, url, **kwargs)
            response.raise_for_status()
            return response
        except httpx.HTTPError as e:
            if attempt < config.max_retries - 1:
                delay = config.retry_delay * (2 ** attempt)  # Exponential backoff
                logger.warning(f"Request failed (attempt {attempt + 1}/{config.max_retries}), retrying in {delay}s...")
                await asyncio.sleep(delay)
            else:
                raise


async def fetch_topics(client: httpx.AsyncClient, config: PipelineConfig,
                      logger: PipelineLogger) -> List[Dict]:
    """Fetch topics from main Locus backend"""

    logger.info(f"Fetching topics from {config.main_backend}/api/topics")

    response = await retry_request(
        client, "GET", f"{config.main_backend}/api/topics",
        config, logger, timeout=30.0
    )

    topics = response.json()

    # Filter enabled topics and subtopics
    enabled_topics = []
    for topic in topics:
        if not topic.get('enabled', True):
            continue

        # Always skip the test topic
        if topic['id'] == 'test':
            continue

        if config.topics_filter and topic['id'] not in config.topics_filter:
            continue

        enabled_subtopics = [
            st for st in topic.get('subtopics', [])
            if st.get('enabled', True)
        ]

        if enabled_subtopics:
            topic['subtopics'] = enabled_subtopics
            enabled_topics.append(topic)

    logger.success(f"Found {len(enabled_topics)} enabled topics")
    return enabled_topics


async def generate_and_save_script(client: httpx.AsyncClient, config: PipelineConfig,
                                   logger: PipelineLogger, stats: PipelineStats,
                                   topic_id: str, subtopic_id: str,
                                   difficulty: str) -> bool:
    """Generate a script and save it"""

    script_name = f"{topic_id}_{subtopic_id}_{difficulty}"

    try:
        # Check if script already exists (skip check when overwriting)
        if not config.overwrite:
            try:
                check_response = await client.get(f"{config.factory_backend}/scripts/{script_name}", timeout=10.0)
                if check_response.status_code == 200:
                    logger.info(f"Script {script_name} already exists, skipping")
                    stats.scripts_generated += 1
                    return True
            except:
                pass  # Script doesn't exist, continue with generation

        # Generate script
        logger.info(f"Generating script: {script_name}")

        if config.dry_run:
            logger.info("  [DRY RUN] Would generate script")
            stats.scripts_generated += 1
            return True

        generate_response = await retry_request(
            client, "POST", f"{config.factory_backend}/generate-script",
            config, logger,
            json={
                "main_topic": topic_id,
                "subtopic": subtopic_id,
                "difficulty_level": difficulty,
                "grading_mode": config.grading_mode
            },
            timeout=config.timeout
        )

        script_data = generate_response.json()
        script_code = script_data['script']

        # Save script
        save_response = await retry_request(
            client, "POST", f"{config.factory_backend}/scripts/save",
            config, logger,
            json={
                "name": script_name,
                "script": script_code,
                "description": f"{topic_id} - {subtopic_id} ({difficulty})",
                "overwrite": config.overwrite
            },
            timeout=30.0
        )

        save_data = save_response.json()
        logger.success(f"  Saved as {save_data['filename']}")
        stats.scripts_generated += 1
        return True

    except Exception as e:
        logger.error(f"  Failed: {str(e)[:100]}")
        stats.scripts_failed += 1
        stats.add_failure("script_generation", script_name, str(e))
        return False


async def generate_all_scripts(client: httpx.AsyncClient, config: PipelineConfig,
                               logger: PipelineLogger, stats: PipelineStats,
                               topics: List[Dict]) -> int:
    """Generate scripts for all topic/subtopic/difficulty combinations"""

    logger.section("STEP 2: Generating Scripts")

    difficulties = ["easy", "medium", "hard"]
    total_combinations = sum(
        len(topic['subtopics']) * len(difficulties)
        for topic in topics
    )

    logger.info(f"Total combinations to generate: {total_combinations}")

    if config.skip_generation:
        # Check existing scripts
        response = await client.get(f"{config.factory_backend}/scripts")
        existing_scripts = response.json()
        logger.info(f"Found {existing_scripts['count']} existing scripts, skipping generation")
        stats.scripts_generated = existing_scripts['count']
        return existing_scripts['count']

    # Build all tasks
    all_tasks = []
    for topic in topics:
        topic_id = topic['id']
        for subtopic in topic['subtopics']:
            for difficulty in difficulties:
                all_tasks.append((topic_id, subtopic['id'], difficulty))

    stats.topics_processed = len(topics)

    # Run with concurrency limit
    semaphore = asyncio.Semaphore(4)
    completed = 0

    async def run_one(topic_id, subtopic_id, difficulty):
        nonlocal completed
        async with semaphore:
            await generate_and_save_script(
                client, config, logger, stats,
                topic_id, subtopic_id, difficulty
            )
            completed += 1
            if completed % 10 == 0:
                logger.info(f"  Progress: {completed}/{total_combinations}")

    await asyncio.gather(*(
        run_one(tid, sid, diff) for tid, sid, diff in all_tasks
    ))

    logger.success(f"\nScript generation complete: {stats.scripts_generated} succeeded, {stats.scripts_failed} failed")
    return stats.scripts_generated


async def mass_generate_problems(client: httpx.AsyncClient, config: PipelineConfig,
                                 logger: PipelineLogger, stats: PipelineStats) -> int:
    """Run mass generation on all scripts"""

    logger.section("STEP 3: Mass Generating Problems")

    if config.dry_run:
        logger.info(f"[DRY RUN] Would generate {config.problems_per_script} problems per script")
        # Estimate problems
        stats.problems_generated = stats.scripts_generated * config.problems_per_script
        return stats.problems_generated

    logger.info(f"Running all scripts {config.problems_per_script} times each...")

    response = await retry_request(
        client, "POST", f"{config.factory_backend}/mass-generate",
        config, logger,
        json={"count_per_script": config.problems_per_script},
        timeout=3600.0  # 1 hour for mass generation (150 scripts × 100 runs)
    )

    result = response.json()
    stats.problems_generated = result.get('total_generated', 0)

    logger.success(f"Mass generation complete: {stats.problems_generated} problems generated")
    logger.info(f"  Scripts run: {result.get('scripts_run', 0)}")
    logger.info(f"  Total executions: {result.get('total_executions', 0)}")

    return stats.problems_generated


async def upload_staged(client: httpx.AsyncClient, config: PipelineConfig,
                        logger: PipelineLogger, stats: PipelineStats):
    """Upload staged problems directly to main Locus backend"""

    logger.section("STEP 4: Uploading Staged Problems to Backend")

    if config.dry_run:
        logger.info("[DRY RUN] Would upload staged problems to backend")
        stats.problems_uploaded = stats.problems_generated
        return

    logger.info("Uploading staged problems to Locus backend...")

    response = await retry_request(
        client, "POST", f"{config.factory_backend}/upload-staged",
        config, logger,
        timeout=1800.0  # 30 minutes for large uploads
    )

    result = response.json()
    stats.problems_uploaded = result['submitted']

    logger.success(f"Upload complete: {result['submitted']}/{result['total']} problems uploaded")

    if result.get('errors'):
        logger.warning(f"  Encountered {len(result['errors'])} errors:")
        for error in result['errors'][:5]:
            logger.warning(f"    {error}")


async def clear_staged_problems(client: httpx.AsyncClient, config: PipelineConfig,
                                logger: PipelineLogger):
    """Clear staged problems before starting"""

    if config.dry_run:
        logger.info("[DRY RUN] Would clear staged problems")
        return

    logger.info("Clearing staged problems...")

    response = await client.delete(f"{config.factory_backend}/staged")
    result = response.json()

    logger.success(result['message'])


async def run_pipeline(config: PipelineConfig, logger: PipelineLogger):
    """Execute the complete automation pipeline"""

    stats = PipelineStats()

    try:
        async with httpx.AsyncClient() as client:
            # Step 0: Clear staged if requested
            if config.clear_before:
                logger.section("STEP 0: Clearing Staged Problems")
                await clear_staged_problems(client, config, logger)

            # Step 1: Fetch topics
            logger.section("STEP 1: Fetching Topics")
            topics = await fetch_topics(client, config, logger)

            if not topics:
                logger.error("No enabled topics found!")
                return stats

            # Step 2: Generate scripts
            scripts_count = await generate_all_scripts(client, config, logger, stats, topics)

            if scripts_count == 0 and not config.skip_generation:
                logger.error("No scripts generated successfully!")
                return stats

            # Step 3: Mass generate problems
            await mass_generate_problems(client, config, logger, stats)

            # Step 4: Upload staged directly (no JSON file)
            await upload_staged(client, config, logger, stats)

    except KeyboardInterrupt:
        logger.warning("\nPipeline interrupted by user")
    except Exception as e:
        logger.error(f"\nPipeline failed with error: {str(e)}")
        import traceback
        logger.error(traceback.format_exc())

    return stats


def main():
    parser = argparse.ArgumentParser(
        description="Locus Factory - Full Pipeline Automation",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )

    parser.add_argument(
        '--skip-generation',
        action='store_true',
        help='Skip script generation if scripts already exist'
    )

    parser.add_argument(
        '--problems-per-script',
        type=int,
        default=100,
        help='Number of problems to generate per script (default: 100)'
    )

    parser.add_argument(
        '--topics',
        type=str,
        default='',
        help='Comma-separated list of topic IDs to process (default: all)'
    )

    parser.add_argument(
        '--overwrite',
        action='store_true',
        help='Overwrite existing scripts instead of adding a timestamp suffix'
    )

    parser.add_argument(
        '--dry-run',
        action='store_true',
        help='Show what would be done without executing'
    )

    parser.add_argument(
        '--clear-before',
        action='store_true',
        help='Clear staged problems before starting'
    )

    parser.add_argument(
        '--main-backend',
        type=str,
        default='http://localhost:3000',
        help='Main Locus backend URL (default: http://localhost:3000)'
    )

    parser.add_argument(
        '--factory-backend',
        type=str,
        default='http://localhost:9090',
        help='Factory backend URL (default: http://localhost:9090)'
    )

    parser.add_argument(
        '--grading-mode',
        type=str,
        default='equivalent',
        choices=['equivalent', 'factor', 'expand'],
        help='Grading mode for all problems (default: equivalent)'
    )

    parser.add_argument(
        '--timeout',
        type=float,
        default=300.0,
        help='LLM request timeout in seconds (default: 300)'
    )

    parser.add_argument(
        '--log-file',
        type=str,
        help='Write detailed logs to specified file'
    )

    args = parser.parse_args()
    config = PipelineConfig(args)
    logger = PipelineLogger(config.log_file)

    # Print configuration
    logger.section("Locus Factory - Pipeline Automation")
    logger.info(f"Main backend: {config.main_backend}")
    logger.info(f"Factory backend: {config.factory_backend}")
    logger.info(f"Problems per script: {config.problems_per_script}")
    logger.info(f"Grading mode: {config.grading_mode}")
    logger.info(f"Topics filter: {config.topics_filter or 'all'}")
    logger.info(f"Skip generation: {config.skip_generation}")
    logger.info(f"Overwrite scripts: {config.overwrite}")
    logger.info(f"Clear before: {config.clear_before}")
    logger.info(f"Dry run: {config.dry_run}")

    if config.dry_run:
        logger.warning("\n*** DRY RUN MODE - No actual changes will be made ***\n")

    # Run pipeline
    stats = asyncio.run(run_pipeline(config, logger))

    # Print summary
    logger.section("Pipeline Complete")
    logger.info(stats.summary())

    elapsed = datetime.now() - logger.start_time
    logger.info(f"\nTotal execution time: {elapsed}")

    # Exit code based on success
    if stats.scripts_failed > 0 or len(stats.failures) > 0:
        sys.exit(1)
    else:
        sys.exit(0)


if __name__ == "__main__":
    main()

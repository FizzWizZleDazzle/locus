#!/usr/bin/env python3
"""One-shot migration: rewrite legacy problem YAMLs into the variants-only schema.

For each .yaml under the target dir(s):
  - If it has no `variants:` key, wrap the whole body in
    `variants: [{name: default, ...body}]`.
  - If it already has `variants:` plus top-level body fields, fill each variant's
    missing fields from the top-level body (replicating the old `merge_variant`
    inheritance), then strip those fields from the top level.

Header keys (kept at top level): topic, difficulty, calculator, time.
Body keys (moved into the variant): variables, constraints, question, answer,
answer_type, mode, format, solution, diagram.

Note: comments are not preserved (PyYAML loses them).
"""

from __future__ import annotations

import argparse
import copy
import sys
from pathlib import Path

import yaml

HEADER_KEYS = ("topic", "difficulty", "calculator", "time")
BODY_KEYS = (
    "variables",
    "constraints",
    "question",
    "answer",
    "answer_type",
    "mode",
    "format",
    "solution",
    "diagram",
)
VARIANT_KEY_ORDER = (
    "name",
    "variables",
    "constraints",
    "question",
    "answer",
    "answer_type",
    "mode",
    "format",
    "solution",
    "diagram",
    "difficulty",
)
TOP_KEY_ORDER = ("topic", "difficulty", "calculator", "time", "variants")


def reorder(d: dict, order: tuple[str, ...]) -> dict:
    out: dict = {}
    for k in order:
        if k in d:
            out[k] = d[k]
    for k, v in d.items():
        if k not in out:
            out[k] = v
    return out


def migrate(doc: dict) -> tuple[dict, str]:
    """Returns (new_doc, status). status: 'wrapped' | 'split' | 'noop' | 'malformed'."""
    if not isinstance(doc, dict):
        return doc, "malformed"

    body = {k: doc[k] for k in BODY_KEYS if k in doc}
    has_variants = "variants" in doc and isinstance(doc["variants"], list)

    if not has_variants:
        if not body:
            return doc, "malformed"
        variant = {"name": "default", **copy.deepcopy(body)}
        new_doc = {k: v for k, v in doc.items() if k in HEADER_KEYS}
        new_doc["variants"] = [reorder(variant, VARIANT_KEY_ORDER)]
        return reorder(new_doc, TOP_KEY_ORDER), "wrapped"

    # Already has variants — fill each variant's missing keys from the top body.
    # Deep-copy so each variant ends up with its own dict tree (no YAML anchors).
    new_variants: list = []
    for i, v in enumerate(doc["variants"]):
        if not isinstance(v, dict):
            return doc, "malformed"
        merged = copy.deepcopy(v)
        if "name" not in merged or not merged["name"]:
            merged["name"] = f"variant_{i+1}"
        for k, val in body.items():
            if k not in merged:
                merged[k] = copy.deepcopy(val)
        new_variants.append(reorder(merged, VARIANT_KEY_ORDER))

    new_doc = {k: v for k, v in doc.items() if k in HEADER_KEYS}
    new_doc["variants"] = new_variants
    status = "split" if body else "noop"
    return reorder(new_doc, TOP_KEY_ORDER), status


class _IndentDumper(yaml.SafeDumper):
    """Force a leading indent on list items so `variants:` items align cleanly."""

    def increase_indent(self, flow=False, indentless=False):  # type: ignore[override]
        return super().increase_indent(flow, False)


def _str_presenter(dumper: yaml.SafeDumper, data: str) -> yaml.ScalarNode:
    if "\n" in data:
        return dumper.represent_scalar("tag:yaml.org,2002:str", data, style="|")
    return dumper.represent_scalar("tag:yaml.org,2002:str", data)


_IndentDumper.add_representer(str, _str_presenter)


def dump(doc: dict) -> str:
    return yaml.dump(
        doc,
        Dumper=_IndentDumper,
        sort_keys=False,
        default_flow_style=False,
        allow_unicode=True,
        width=10000,
    )


def process_file(path: Path, dry_run: bool) -> str:
    raw = path.read_text()
    try:
        doc = yaml.safe_load(raw)
    except yaml.YAMLError as e:
        return f"PARSE_ERROR {path}: {e}"
    new_doc, status = migrate(doc)
    if status == "noop":
        return f"NOOP       {path}"
    if status == "malformed":
        return f"MALFORMED  {path}"
    if not dry_run:
        path.write_text(dump(new_doc))
    return f"{status.upper():10} {path}"


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("paths", nargs="+", type=Path, help="Files or dirs to migrate.")
    p.add_argument("--dry-run", action="store_true")
    args = p.parse_args()

    files: list[Path] = []
    for root in args.paths:
        if root.is_file():
            files.append(root)
        else:
            files.extend(sorted(root.rglob("*.yaml")))

    counts: dict[str, int] = {}
    for f in files:
        result = process_file(f, args.dry_run)
        tag = result.split(maxsplit=1)[0]
        counts[tag] = counts.get(tag, 0) + 1
        print(result)

    print(f"\n=== {sum(counts.values())} files; { ', '.join(f'{k}={v}' for k, v in counts.items()) }")
    return 0


if __name__ == "__main__":
    sys.exit(main())

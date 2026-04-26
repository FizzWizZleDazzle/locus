"""Fetch real AIME problems from AoPS wiki, write to JSON."""
import json
import re
import sys
import time
from pathlib import Path

import requests
from bs4 import BeautifulSoup, NavigableString

UA = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/120 Safari/537.36"
HEADERS = {"User-Agent": UA}
BASE = "https://artofproblemsolving.com/wiki/index.php"
OUT = Path("/home/artur/Locus/scripts/daily_puzzles/aime_fetched.json")
SESSION = requests.Session()
SESSION.headers.update(HEADERS)

TOPIC_HINTS = [
    (re.compile(r"triangle|circle|polygon|tangent|angle|hexagon|sphere|tetrahedron|trapezoid|parallelogram|inradius|circumradius", re.I), ("geometry", "geometry")),
    (re.compile(r"probability|expected|random|coin|dice|chosen at random", re.I), ("precalculus", "probability")),
    (re.compile(r"divisor|prime|modulo|remainder|integer|factor|gcd|lcm|digits", re.I), ("arithmetic", "number_theory")),
    (re.compile(r"sequence|recursion|recurrence|fibonacci", re.I), ("algebra2", "sequences")),
    (re.compile(r"polynomial|root|coefficient|x\^2|complex", re.I), ("algebra2", "polynomials")),
    (re.compile(r"log|exponent|trig|sin|cos|tan", re.I), ("precalculus", "trigonometry")),
    (re.compile(r"permutation|combination|arrangement|choose|subset|sequence of letters", re.I), ("precalculus", "combinatorics")),
]


def topic_for(text: str):
    for rx, t in TOPIC_HINTS:
        if rx.search(text):
            return t
    return ("algebra2", "miscellaneous")


def fetch(url: str, retries=3) -> str | None:
    for i in range(retries):
        try:
            r = SESSION.get(url, timeout=20)
            if r.status_code == 200:
                return r.text
            print(f"  HTTP {r.status_code} for {url}", file=sys.stderr)
        except Exception as e:
            print(f"  err {e} for {url}", file=sys.stderr)
        time.sleep(1 + i)
    return None


def latex_replace(content):
    """Replace <img class=latex alt="$...$"> with inline LaTeX text."""
    for img in content.find_all("img", class_="latex"):
        alt = img.get("alt", "")
        img.replace_with(NavigableString(alt))


def extract_problem_text(html: str) -> str | None:
    soup = BeautifulSoup(html, "html.parser")
    content = soup.find("div", class_="mw-parser-output")
    if not content:
        return None
    latex_replace(content)
    # find Problem heading; collect siblings until next heading
    headings = content.find_all(["h1", "h2", "h3"])
    problem_h = None
    for h in headings:
        if h.get_text(strip=True).lower().startswith("problem"):
            problem_h = h
            break
    if not problem_h:
        return None
    parts = []
    for sib in problem_h.next_siblings:
        if getattr(sib, "name", None) in ("h1", "h2", "h3"):
            break
        if isinstance(sib, NavigableString):
            parts.append(str(sib))
        else:
            parts.append(sib.get_text(" "))
    text = " ".join(parts)
    text = re.sub(r"\s+", " ", text).strip()
    return text or None


def extract_first_solution(html: str) -> str | None:
    soup = BeautifulSoup(html, "html.parser")
    content = soup.find("div", class_="mw-parser-output")
    if not content:
        return None
    latex_replace(content)
    headings = content.find_all(["h1", "h2", "h3"])
    sol_h = None
    for h in headings:
        if h.get_text(strip=True).lower().startswith("solution"):
            sol_h = h
            break
    if not sol_h:
        return None
    parts = []
    for sib in sol_h.next_siblings:
        if getattr(sib, "name", None) in ("h1", "h2", "h3"):
            break
        if isinstance(sib, NavigableString):
            parts.append(str(sib))
        else:
            parts.append(sib.get_text(" "))
    text = re.sub(r"\s+", " ", " ".join(parts)).strip()
    if len(text) > 800:
        text = text[:800].rsplit(".", 1)[0] + "."
    return text or None


def fetch_answer_key(year: int, contest: str) -> list[str] | None:
    url = f"{BASE}/{year}_AIME_{contest}_Answer_Key"
    html = fetch(url)
    if not html:
        return None
    soup = BeautifulSoup(html, "html.parser")
    content = soup.find("div", class_="mw-parser-output")
    if not content:
        return None
    nums = re.findall(r"\b(\d{1,3})\b", content.get_text("\n"))
    # filter to plausible: keep first 15 that look like 0-999 answers; skip leading nav numbers
    # heuristic: answers come BEFORE the problem-list block "1 • 2 • 3 ..."
    text = content.get_text("\n")
    # find marker
    cut = text.find("Preceded by")
    if cut > 0:
        text = text[:cut]
    nums = re.findall(r"\b(\d{1,3})\b", text)
    if len(nums) < 15:
        return None
    return [n.zfill(3) if len(n) < 3 else n for n in nums[:15]]


def derive_title(qtext: str) -> str:
    # heuristic: first noun phrase
    # use first sentence's first 5 words
    first = re.split(r"[.!?]", qtext, 1)[0]
    words = first.split()[:5]
    return " ".join(words).rstrip(",;:") or "AIME Problem"


def main():
    years = list(range(2010, 2025))  # 2010..2024
    contests = ["I", "II"]
    out = []
    for year in years:
        for contest in contests:
            print(f"=== {year} AIME {contest} ===")
            answers = fetch_answer_key(year, contest)
            if not answers:
                print(f"  ans key fail")
                continue
            for n in range(1, 16):
                url = f"{BASE}/{year}_AIME_{contest}_Problems/Problem_{n}"
                html = fetch(url)
                if not html:
                    print(f"  P{n} fetch fail")
                    continue
                qtext = extract_problem_text(html)
                if not qtext:
                    print(f"  P{n} parse fail")
                    continue
                sol = extract_first_solution(html) or ""
                ans = answers[n - 1].lstrip("0") or "0"
                main_topic, subtopic = topic_for(qtext)
                title = derive_title(qtext)
                out.append({
                    "question_latex": qtext,
                    "answer_key": ans,
                    "editorial_latex": sol,
                    "hints": [],
                    "title": title,
                    "source": f"{year} AIME {contest} #{n}",
                    "year": year,
                    "contest": f"AIME {contest}",
                    "number": n,
                    "difficulty": 2400 + 100 * (n - 1),
                    "main_topic": main_topic,
                    "subtopic": subtopic,
                    "answer_type": "numeric",
                    "grading_mode": "equivalent",
                    "calculator_allowed": "none",
                })
                print(f"  P{n} ok ans={ans}")
                time.sleep(0.4)
    OUT.write_text(json.dumps(out, indent=2, ensure_ascii=False))
    print(f"\nWrote {len(out)} problems to {OUT}")


if __name__ == "__main__":
    main()

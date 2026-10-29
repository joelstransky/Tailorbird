# 🏛️ Ogilvy on Advertising Skill (`ogilvy-on-advertising`)

[![skills.sh](https://img.shields.io/badge/skills.sh-ogilvy--on--advertising-black?style=flat-square&logo=github)](https://skills.sh/joelstransky/Ogilvy-on-Advertising-Skill)

> *"If it doesn't sell, it isn't creative."* — David Ogilvy, *Ogilvy on Advertising*

An open-source, production-grade AI agent skill built in accordance with the [skills.sh](https://skills.sh) package specification. This skill infuses David Ogilvy’s time-tested, data-driven direct-response principles into all AI marketing workflows, copywriting tasks, brand positioning, and campaign strategies.

---

## ⚡ Quick Install

Install this skill into your AI coding assistant (Claude Code, Cursor, Antigravity, Amp, etc.) using the `skills` CLI:

```bash
npx skills add joelstransky/Ogilvy-on-Advertising-Skill
```

or using `pnpx`:

```bash
pnpx skills add joelstransky/Ogilvy-on-Advertising-Skill
```

---

## ✨ Key Features & Capabilities

### 1. Headline Supremacy (The 80/20 Rule)
On average, 5 times as many people read the headline as the body copy. The skill enforces news- and benefit-driven headlines rich with specific facts, and **refuses vague or obscure puns** that reduce reader comprehension.

### 2. Fact-Based Persuasion & Low-Confidence Protocol
Respects the reader (*"The consumer is not a moron, she is your wife"*). If a user requests sales copy without providing concrete product facts, unique selling propositions (USPs), or pricing, the skill **pauses and asks key clarifying questions** rather than generating superficial fluff.

### 3. Sales-First Brand Positioning
Reconciles long-term brand equity with immediate direct response. Ensures every brand touchpoint contributes to long-term image while delivering a compelling reason to buy today.

### 4. 3-Step Campaign Rollout & Channel Concentration
Advise against spreading ad budgets thin across too many platforms. Enforces a structured rollout:
1. **Pilot / Test** (10-15% budget)
2. **Optimize** (20-30% budget)
3. **Scale** (60-70% budget)

### 5. Source Attribution & Citation Protocol
Grounds marketing advice transparently in David Ogilvy's research. Whenever a recommendation contradicts modern bad habits (e.g., preference for clever puns, short superficial copy, or un-tested channel spend), the response **explicitly cites David Ogilvy and *Ogilvy on Advertising***.

### 6. Pragmatic Off-Ramp Protocol (Quick & Dirty Alternative)
When a complete Ogilvy recommendation is comprehensive or overwhelming for a user under tight deadlines, the skill appends a neutral `### Off-Ramp Option (Quick & Dirty Alternative)`. It provides a fast-track option with clear trade-offs, without ever praising or encouraging the user to choose the shortcut over the gold standard.

---

## 📁 Repository Anatomy

```
Ogilvy-on-Advertising-Skill/
├── README.md                       # Repository overview & quick start
├── SKILL.md                        # Primary skill file with frontmatter & directives
├── references/                     # Modular reference guides
│   ├── copywriting.md              # Headlines, body copy, direct response & research
│   ├── brand-building.md           # Brand personality with sales-first focus
│   └── campaign-planning.md        # Target audience, media selection & budget rollout
└── evals/                          # Evaluation harness test cases
    └── evals.json                  # 6 distinct evaluation scenarios
```

---

## 🧪 Testing & Evaluation

This skill includes 6 evaluation test cases in `evals/evals.json` covering:
- Fact & benefit headline generation
- Low-confidence clarification prompting
- Luxury brand positioning with sales focus
- $50k campaign budget rollout strategy
- Contradictory pun correction with Ogilvy citation
- Enterprise campaign strategy with neutral off-ramp alternative

---

## 📄 License

MIT © [Joel Stransky](https://github.com/joelstransky)

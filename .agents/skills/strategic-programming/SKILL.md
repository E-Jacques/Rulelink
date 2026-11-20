---
name: strategic-programming
description: Strategic programming describes how to design code so that it stays easy to change over time. Use when planning a feature, extracting shared code, or deciding how much structure a change deserves.
---

# Strategic programming

## The lore

It's war, everywhere. You are the general leading your troops from battle to battle. Your winning streak is impressive, a real record breaker, but you are cheap with your troops. They start to flee the war and sabotage your operations. Even if you win every battle, small or big, you may lose the war in the long run.

Except you are not a general and you have no troops. As an agent, what you have is a code base, a lot of functions, and some features to develop. The same principle applies: if you are cheap on taking care of them, they will turn against you and block you.

## How to use this skill

Every principle below ends with a **deliverable**. A principle without its deliverable has not been applied. Deliverables go in the plan note (`.scratch/<feature>/plan-<NN>.md`) before coding, and in the review report after.

Apply the principles in order.

### 1. Plan

Planning applies design at a global scale while keeping the big picture in mind. Before writing code, answer these in the plan note, in a few lines each:

1. **Interface.** What is the signature the caller sees? Write it.
2. **Misuse.** What can the caller get wrong: call order, partial failure, forgotten step? If the answer is anything, change the interface until the wrong use is impossible. Prefer one deep operation (allocate, validate and insert in one call) over several steps the caller must chain.
3. **Ownership.** For each decision (parsing, validation, defaults, error class), which layer owns it? Domain rules live in the model. The application layer orchestrates. The CLI parses arguments and prints.
4. **One sentence.** Describe the application function in one sentence without "and". If you cannot, it does too much.

Then check the design against this smell list and say which smells you found and fixed:

- **Temporal coupling**: two calls that only work in a given order.
- **State mutated before validation**: a failure leaves something half-changed.
- **Primitive obsession at a boundary**: `String` where the model has a type (`Origin`, `RuleId`).
- **Domain logic in `app` or `cli`**: parsing or rules that another command will need.
- **Wrong error class**: user input reported as a configuration error, or the reverse.
- **Catch-all error**: one enum for unrelated causes, `#[non_exhaustive]` on a workspace error, a `_ =>` arm or a blanket `From` that files every variant under one class, or a variant whose name is narrower than what it wraps. Split the type by cause and match every variant.

Read `codebase-design` for the vocabulary (deep module, seam, interface) whenever you add or change a function that other code will call, not only for new modules.

**Deliverable:** the plan note.

### 2. Do not repeat yourself (DRY)

Simple to say, not always easy to apply.

1. Identify whether the two repeated pieces of code implement the same business logic, or simply share an algorithm that should be generalised and extracted.
2. If it is an algorithm, take a step back and rephrase the problem it solves, so that the extraction handles the use cases it may face.
3. Include repetition with the code that already exists in the repository, not only within your diff: search for the same `map_err`, path handling or temp-file logic before writing it again.

**Deliverable:** a line in the plan note listing what you looked for and what you reused or extracted.

### 3. Follow the roadmap

The roadmap is your best friend. Build each feature while keeping in mind what comes next and how the code you write now may affect upcoming implementations.

Do this with commands, not intuition:

1. List the tickets that depend on yours: `grep -l "Blocked by:.*<NN>" .scratch/*/issues/*.md`.
2. For each one, write what it will reuse from your change and what it will need that you do not provide.
3. Whenever you defer a problem ("out of scope", "unrealistic"), check it against that list. If a dependent ticket hits the same problem, it is not out of scope: fix it, or record which ticket inherits it.

**Deliverable:** a table in the plan note: dependent ticket, what it reuses, what is missing.

### 4. Write self-documenting code

Think about the next programmer who will read this code. Will this piece of code be understandable without you around to explain it?

- Names say what the code does, including side effects such as writing to disk.
- Comments explain why, especially for invariants the types do not carry.
- A test is named after the behaviour it guarantees, and checks one thing.

**Deliverable:** none; the review checks it.

## Before handing over

State in one line each: smells found and fixed, what you extracted or reused, which dependent tickets you checked. If there is nothing to refactor, say why instead of saying nothing.

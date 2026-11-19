---
name: strategic-programming
description: Strategic programming describes how to design code so that it stays easy to change over time. Use when planning a feature, extracting shared code, or deciding how much structure a change deserves.
---

# Strategic programming

## The lore

It's war, everywhere. You are the general leading your troops from battle to battle. Your winning streak is impressive, a real record breaker, but you are cheap with your troops. They start to flee the war and sabotage your operations. Even if you win every battle, small or big, you may lose the war in the long run.

Except you are not a general and you have no troops. As an agent, what you have is a code base, a lot of functions, and some features to develop. The same principle applies: if you are cheap on taking care of them, they will turn against you and block you.

## The principles

Apply these principles in order to keep the code clean in the long run.

### 1. Plan

Planning lets you apply design patterns at a global scale while keeping the big picture of what you are building in mind.

### 2. Do not repeat yourself (DRY)

Simple to say, not always easy to apply.

1. Identify whether the two repeated pieces of code implement the same business logic, or simply share an algorithm that should be generalised and extracted.
2. If it is an algorithm, take a step back and rephrase the problem it solves, so that the extraction handles the use cases it may face.

### 3. Follow the roadmap

The roadmap is your best friend. Build each feature while keeping in mind what comes next and how the code you write now may affect upcoming implementations. Easy to say, hard to master.

- Look for similarities or patterns across similar features.
- Ask yourself: "Will I need this piece of code again in the next 3 features I will develop?"

### 4. Write self-documenting code

Think about the next programmer who will read this code. Will this piece of code be understandable without you around to explain it?

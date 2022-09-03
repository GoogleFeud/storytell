
# Paths

The main construct of the language are **paths** - they're like different sections of your story which can play out differently, depending on the choices the reader makes. Each story starts at a path and ends at another. Paths contain children called **blocks** which do different things - one kind of block for example is the paragraph, which just shows text to the reader. Each path can also have other paths as children, which can be used for different versions or different sections of the path.

Here is an example:

```md
# Chapter 1, Part 2

This is a paragraph, and it's shown when the reader arrives at this path. You **can** *use*  __markdown__ to style your text.

// This is a comment, the arrow below will divert the reader to the "Diversion 1" path, which is a children of this path
-> diversion_1

## Diversion 1

This is a diversion from the main path. The reader can come here from anywhere, inclduding from the main path.
```

## Diversions

Paths can divert you to other paths with the `->` syntax. The path transfers the control flow to another path. The reader never returns to this path unless another path specifically diverts the reader to it.

A path is considered to "end" when:

- It diverts to the `end` path.
- There's no more content to be shown.

### Diverting to children paths

Main paths (Those which are created with the `#` symbol) have to be unique - there can't be multiple paths with the same name, however, path children can!

A path can divert to it's children directly:

```md
# Main Path

## Child 1

// Works!
-> child_of_child

### Child of child

## Child 2
```

However, if the path you want to divert to is not a **direct** child of the current path, you have to use the **access** syntax to reach it:

```md
# Main Path

## Child 1

// Doesn't work
-> child_2

// Works!
-> main_path.child_2

### Child of child

## Child 2
```
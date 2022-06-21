
# Storytell

Storytell is a text editor which allows you to write interactive fiction - from scripts with simple choices to massive stories with many branching storylines and endings. It also allows you to **play** your story as you write it, and view how your stories branch. It's not meant for game creation - only writing scripts for games or movies.

## The language

Storytell allows you to write your scripts in a simple markup language which has full markdown support. Here is a very simple example:

```md
# Chapter 1, Part 2

This text is always shown when the reader arrives at this path. You can use **plain** __markdown__ to style your text.

// This is a comment, the arrow below will divert the reader to the "Diversion 1" path
-> diversion_1

## Diversion 1

This is a diversion from the main path. The reader can come here from anywhere, inclduding from the main path.
```

### Paths

In the above example, we have two **paths**. Paths are the main structure of the language - the reader starts at a path and ends at another path. Paths can be nested. Paths can divert you to other paths with the `->` symbol. There are a few types of diversion:

- `->` - Direct diversion. The path transfers the flow control to another path. The reader never returns to this path unless another path specifically directs them at the same path.
- `<->` - Temporary diversion. After the path ends, the reader continues from the old path.

A path is considered to "end" when:

- It diverts to the `EXIT` path.
- There's no more content to be shown.

So if the path diverts to another path, the temporary diversion will wait for that path to end, too! Things can get a little confusing with temporary diversion so use it sparingly!

### Choices

**Choices** is what the reader makes in order to end up in different **paths**:

```md
# Graveyard

James and Alicia arrive at the graveyard. Where should they investigate first?

- Left -> graveyard_left
- Right -> graveyard_right

## Graveyard Left

James and Alicia decided to investigate the left area of the graveyard first. They found nothing useful.

## Graveyard Right

James and Alicia found an open grave!
```

The above example uses a **choice group** to ask the reader on what to do. Only one option can be chosen. The choice group ends after a new empty line.

If the reader chooses `Left`, they'll divert to the `Graveyard Left` path, and if they choose `Right`, they'll divert to the `Graveyard Right` path, and afterwards the story will end. 

But what if we wanted to make it so the reader returns to the start after they explore one of the paths, so they can explore both the `Left` and `Right` path? We use temporary diversion!

```md
- Left <-> graveyard_left
- Right <-> graveyard_right
```

But this still isn't quite right, because the reader is stuck - they'll always end up in the middle, choosing between left and right. We can make each option be selectable only once with **option attributes**:

```md
- #[once] Left <-> graveyard_left
- #[once] Right <-> graveyard_right

You investigated both paths.
```

Now, after the player visits both paths, they'll see the "You investigated both paths" and the game will end! Cool! But what if we made it so it's optional to visit both paths? We will create a third option called "Leave graveyard" which ends the chapter:

```md
- #[once] Left <-> graveyard_left
- #[once] Right <-> graveyard_right
- Leave Graveyard -> END
```

#### Choice children

The last example with the graveyard was excessive - we do not really need two separate paths just do display different text. We can put the text directly below the option:

```md
#[exaust]
- Left
    James and Alicia decided to investigate the left area of the graveyard first. They found nothing useful.
- Right
    James and Alicia found an open grave!
- Leave Graveyard -> END
```

We can also use the `exaust` attribute, which lets the reader choose until all of the options have been checked out. This is **better** than temporary diversion because the reader won't return to this option if one of the inner options change the path.

### Embedded Javascript

You can also use **javascript** inside your paths, to save values into variables, or run any arbitrary code!

```md
## Animal Encounter

You encounter an animal. Kill it?

- Kill
    { animalsKilled += 1 }
    You killed the animal. You have now killed {animalsKilled} animals.
- Spare
```




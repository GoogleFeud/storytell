
# Paths

The main construct of the language are **paths** - they're like different sections of your story which can play out differently, depending on the choices the reader makes. Each story starts at a path and ends at another. Each path can also have **deviations**, which can also be used as sub-paths. Here is an example:

```md
# Chapter 1, Part 2

This text is shown when the reader arrives at this path. You **can** *use*  __markdown__ to style your text.

// This is a comment, the arrow below will divert the reader to the "Diversion 1" path
-> diversion_1

## Diversion 1

This is a diversion from the main path. The reader can come here from anywhere, inclduding from the main path.
```

## Diversions

Paths can divert you to other paths with the following symbols:

- `->` - Direct diversion. The path transfers the flow control to another path. The reader never returns to this path unless another path specifically divers the reader to it.
- `<->` - Temporary diversion. After the path ends, the reader continues from the old path.

A path is considered to "end" when:

- It diverts to the `END` path.
- There's no more content to be shown.

So if the path diverts to another path, the temporary diversion will wait for that path to end, too! Things can get a little confusing with temporary diversion so use it sparingly!

```md
# Graveyard

Zach and Betty finally reached the graveyard. 

<-> Left
-> Right

This will never be seen...

## Left

Zach and Betty decided to visit the left section of the graveyard first. They didn't find anything interesting.

## Right

Then they visited the rest of the graveyard. There, they found the unthinkable...
```

This would result in the following:

```
Zach and Betty finally reached the graveyard. 
Zach and Betty decided to visit the left section of the graveyard first. They didn't find anything interesting.
Then they visited the rest of the graveyard. There, they found the unthinkable...
```

### Diverting to sub-paths outside the main path

Main paths (Those which are created with the `#` symbol) have to be unique - there can't be multiple paths with the same name, however, deviations can!

So, let's say the reader is in another path, not `Graveyard`, but we want them to see the contents of the `Left` chapter, which **is** inside Graveyard. We can use a dot `.` to connect both names so Storytell knows where to find the `Left` path:

```
-> Graveyard.Left
// or
-> graveyard.left
// path names are case insensitive!
```
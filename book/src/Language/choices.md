
# Choices

**Choices** are what the reader makes in order to end up in different **paths**:

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

The above example uses a **choice group** to ask the reader on what to do. Only one option can be chosen. After a choice has been made, the choice isn't getting saved anywhere - so if another path takes the reader to the same choice, the reader will be given the same options.

If the reader chooses `Left`, they'll divert to the `Graveyard Left` path, and if they choose `Right`, they'll divert to the `Graveyard Right` path, and afterwards the story will end. 

## Inline responses

A choice doesn't always have to lead to a new path - in the example above, the two different paths are excessive - we can put the text right below the option:

```md
- Left
    James and Alicia decided to investigate the left area of the graveyard first. They found nothing useful.
- Right
    James and Alicia found an open grave!
```

The text below can contain anything - even more choices, but **it has to be on a new line and have at least 4 whitespaces at the start of the new line, depending on how nested the choices are**. 

Here's what it would look like with nested choices:

```md
- Left
    James and Alicia decided to investigate the left area of the graveyard first. They found nothing useful.
- Right
    James and Alicia found an open grave!
    - Take a peek
        They looked inside the grave, and saw the unthinkable.
    - Move on
        Too afraid to look what's inside the grave, they moved on.
```

## Labeling choices and choice groups

Just how you can divert to paths, you can also divert to specific choices and choice groups. You can give them a **label** via attributes. Then, you can use the divert syntax (`->`) to go to that specific choice group or choice.

If you divert to a choice, the content below the choice will be shown. If you divert to a choice group, the reader will have to make the choice. After the divert, the flow goes back to the content **after** the divert, **not** after the choice.

```md
What's 2 + 2?

#[Label(MyChoice)]
- #[Label(Choice1)] 4
    That's correct.
- #[Label(Choice2)] 5
    That's incorrect. Let's try again.
    -> MyChoice
```

You can also check how many times a labeled choice got selected in inline JS and do stuff based on that:

```md
Welcome back! ++

@{!!label_count("Choice1")}
- {true}
    You got the right answer! Congrats!
- {false}
    You did not get the right answer. Let's try again.
    -> MyChoice
```

## Option attributes

**Attributes** can change how an option or an option group behaves.

### once

You can prefix a choice with the `once` attribute, so if the reader ever encounters the same choice group, they can't make the same choice twice:

```md
- #[once] Left
    You chose the left path. You will now be brought back to the same choice.
    -> BACK
- Right
    You went right!
    -> END
```

In the example above, if the reader chooses `Left`, they will see the text below it, and go back to the choice (because of the special `BACK` path), except this time they won't have the `Left` choice anymore, only `Right`.

### exaust

You can add the `exaust` attribute to a choice group. After a choice is made, the reader will be brought back to the choice group, until all of the choices are selected, or until one of the choices takes the reader to another path.

```md
#[exaust]
- Left
    You went left!
- Right
    You went right!
- Exit graveyard
    You exited the graveyard.
    -> END
```

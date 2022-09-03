# Labels

Almost all blocks in storytell (paragraphs, choices, choice groups, match, javascript block) can be labeled. Just how you can divert to paths, you can also divert to specific blocks. You can give them a **label** via attributes. Then, you can use the divert syntax (`->`) to go to that specific block.

```md

#[Label(TheParagraph)]
This is a paragraph, it's label is "TheParagraph".

// Diverting to the paragraph will show it again!
-> TheParagraph

You saw the paragraph twice!
```

After the divert, the flow goes back to the content **after** the divert, **not** after the block. So, in the above example, the reader will see the labeled paragraph twice, and then they'll see `You saw the paragraph twice!`.

- If you divert to a `paragraph`, the paragraph will be shown.
- If you divert to a `choice group`, the reader will be made to make the choice again.
- If you divert to a `single choice`, the choice's children will be shown, as if the choice was picked by the reader.
- If you divert to a `match`, the condition will be matched again.
- If you divert to a `javascript block`, the code will be executed.

## Using labels in InlineJs

Labeling blocks has another advantage - you'll be able to gather information you can use in your story. You don't need to use variables to check which choice has been chosen - you can put a label on it and call the `count` function to see how many times it's children have been shown to the reader. Generally, you should only do this for minor branching in the dialogue.

```md
What's 2 + 2?

#[Label(MyChoice)]
- #[Label(Choice1)] 4
    That's correct.
- #[Label(Choice2)] 5
    That's incorrect.

// Later on...
Oh yeah, I found out ++

@{!!count("Choice1")}
- {true}
    you got the answer! Nice.
- {false}
    you couldn't get the answer... That's a bummer. Maybe you can try again?
    -> MyChoice
```

## Label scopes

By default, labels can only be diverted to if they are in the same (or parent) path as the divertion itself. You have to specify a **scope** if you want to divert to labels outside of the current path. The scope can be either `global`, `local`, or a number which specifies how many levels above the current path the label can be accessed. The default value is `local`.

Scopes are needed because label names are **unique**.

```md
# Main path

// Doesn't work, first_para is in "Second Path"
-> first_para

// Works!
-> second_para

// Works!
-> third_para

## Second Path

#[Label(First Para)]
This is a labeled paragraph.

// Puts the label in the global context
#[Label(Second Para, global)]
This is the second paragraph.

// Puts the label in the "Main path"
#[Label(Third Para, 1)]
This is the third paragraph.

### Nested Path

// Works! All 3 work!
-> first_para

## Third Path

// Doesn't work
-> first_para 
```
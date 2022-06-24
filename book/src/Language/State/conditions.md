
# Conditions

You can show text conditionally in two ways:

## Javascript

You can use javascript's [ternary operator](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Conditional_Operator):

```
{variable === 42 ? "You have found the answer!" : "Wrong answer!"}
{!killed.includes("Nick") && "Nick: Wow, this place is a shithole"}
```

This is okay for simple text like the one above, but if you want to go to a different path or let the reader make a choice, this won't do.

## Match syntax

Storytell includes simple syntax which allows you to compare variables and do different things depening on the value:

```
@{variable}
- {42}
    You have found the answer!
- {}
    Wrong answer!

@{killed.includes("Nick")}
- {true}
    Nick: Wow, this place is a mess. What are we going to do here?
    - We need to find the treasure...
        Nick: I don't think such a treasure could be hidden in a such a hideous place...
    - Shut up Nick.
        ...
```

### :if and :not

The second example with the match syntax takes too much space for something so simple. We can shorten it with `:not` inside the curly brackets:

```
@{:not killed.includes("Nick")}
    Nick: Wow, this place is a mess. What are we going to do here?
    - We need to find the treasure...
        Nick: I don't think such a treasure could be hidden in a such a hideous place...
    - Shut up Nick.
```
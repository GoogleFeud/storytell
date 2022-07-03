
# State

Some choices don't have consequences which change the entire course of the story - they could just change dialogue options, interactions between characters, or possibly what happens in a certain path. We can use **state** for this with **javascript**. [Javascript](https://developer.mozilla.org/en-US/docs/Web/JavaScript) is a scripting language originally meant for the web, but these days it's used everywhere! 
Don't worry, you don't need to know javascript to add state to your story.

Javascript can be used inline by placing it between curly brackets `{}`, or as a block with 3 backticks:

````
{"This is inline javascript"}

```
"This is block javascript"
```
````

## Inline javascript

The inline javascript always evaluates to a value. If this value can be represented as text, it'll be embedded into your story. For example:

```
{"This is inline javascript"}
{[1, 2, 3]}
{3.14}
{true}
{myValue = 1}
{myValue}
```

Will show the following to the reader:

```
This is inline javascript
1, 2, 3
3.14
true
1
```

By default all value assignment (`=`, `+=`, `-=`, etc.) will be hidden, even if it does evaluate to a value that can be represented as text.

## Block javascript

By default, block javascript will never embed anything into your story, unless you make it so by using the `return` keyword, or the `document` API to modify the DOM.

````
```js
const yourName = prompt("What's your name?");
return `Hello, ${yourName}`;
```
````

Will embed:

```
Hello GoogleFeud!
```

# State

Some choices don't have consequences which change the entire course of the story - they could just change dialogue options, interactions between characters, or possibly what happens in a certain path. 

Storytell allows you to save data via a powerful scripting language called `InlineJs`, which is almost exactly the same as [JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript). You **do not** need to know JavaScript in order to use this feature, storing data is super simple!

## Inline usage 

This scripting language can only be used **inline**, by placing it between curly brackets (`{}`). Everything in the language evaluates to a value which will show up in your story, except for **variable assignments**, which completely dissapear.

```
- Action A
    {chosen_action = "A"}
- Action B
    {chosen_action = "B"}

You chose {chosen_action}.
```

will show up as...

```
- Action A
- Action B
[Chooses A]
You chose A.
```

In the above example, we save the string `"A"` to the `chosen_action` variable if the reader chooses the `Action A` option, and if they choose the `Action B` option, `chosen_action` gets set to `"B"`. Afterwards, we show which option the user chose by displaying the contents of the variable.

## Block JavaScript

You can also use regular JavaScript with code blocks, although it should be used only where you can't achieve something with InlineJs, which should be rare.

By default, block JavaScript will never embed anything into your story, unless you make it so by using the `return` keyword, or the `document` API to modify the DOM.

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

## Differences between InlineJS and JavaScript

- Since InlineJS needs to be wrapped between curly brackets (`{}`), object literals are not part of the language.
- Function expressions (`function() {}` and `() => {}`) don't exist in InlineJS.
- In string template literals, inserting a literals happens with `$()`, not `${}`.
- Bitwise operators aren't supported.
- BigInt literals aren't supported.
- Regex literals aren't supported.
- Expression list expressions (`(exp, exp, exp)`) aren't supported.
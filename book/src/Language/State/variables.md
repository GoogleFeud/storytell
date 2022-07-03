# Variables

Your state will usually be saved in **variables**, which are persistent across paths. You can save pretty much anything.

```md
- Choose Ashley
    {chosen = "Ashley"}
- Choose Nick
    {chosen = "Nick"}

{chosen}: Thank you for choosing me.
```

After the user makes their choice, the `chosen` variable gets updated to either `Ashley` or `Nick`, and the character you chose says "Thank you for choosing me".

Here's a cheatsheet:

| Code  | What it does |
|---------|--------|
| `variable = value` | Sets the variable to `value`. |
| `variable++` | Increments the variable. So if it's currently `1`, after the code is ran it will become 2. |
| `variable--` | Decrements the variable. |
| `variable += n` | Adds `n` to the variable. |
| `variable -= n` | Subtracts `n` from the variable. | 
| `variable.push(value)` | Adds a new value to the `variable` list. |
| `variable.set(key, value)` | Adds a new key-value pair to the `variable` table. |

Here's an example of using a list:

```
- Kill Ashley
    {killed.push("Ashley")}
    You killed Ashley.
- Kill Nick
    {killed.push("Nick")}
    You killed Nick.
```

## Variables under the hood

Variables which are used in inline javascript are sort of magic - you don't need to declare them, storytell can automatically detect what the variable is by how you use it and it automatically intializes it. You can use these magic variables in code blocks too, but **you cannot use variables defined in code blocks in inline javascript**. If you'd like to bypass this, you can attach the variable to the **window** object:

````
```js
window.value = 123;
```

{window.value}
````
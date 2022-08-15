# Data

Your state is **data** that is stored in **variables**. Every data has a **type**. For example, `"String"` is a `string` type, `123` is a `number` type, etc. There are also types which can hold multiple different types, for example the `array` type can contain `number`s, `string`s, or maybe even other `array`s.

## Initializing variables

Variables get initialized automatically depending on how you use them.

| Code | Initializes to |
|---------|--------|
| `variable += 1`, `variable -= 1`, `variable *= 1`, `variable /= 1` | `variable` is a number, and gets initialized as `0`. <br>`+=` - Adds a number to the variable.<br>`-=` - Subtracts a number from the variable.<br>`*=` - Multiplies the variable by the number.<br> `/=` - Divides the variable by the number. |
| `variable++`, `variable--` | `variable` is a number, and gets initialized as `0`.<br>`++` - Increments the variable.<br>`--` - Decrements the variable. |
| `variable += "Hello"` | `variable` is a string, and gets initialized as `""`.<br>`+=` - Appends a string to the variable. |
| `variable.push(value)` | `variable` is a list (array) and gets initialized as `[]`.<br>`.push(value)` pushes the value to the list. The value can be anything. You can also provide multiple values - `.push("Hello", 123, true, [])` |
| `variable.key = value` | `variable` is a table (object) and gets initialized as `{}`.<br>`.key = value` adds a new key-value pair to the table. For example, `variable.name = "Google"` is going to set the `name` key to `"Google"`. You can retrive the value inside the key with `variable.name`. |

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

Variables which are used in InlineJS are sort of magic - you don't need to declare them, storytell can automatically detect what the variable is by how you use it and it automatically intialize it. You can use these magic variables in code blocks too, but **you cannot use variables defined in code blocks in inline javascript**. If you'd like to bypass this, you can attach the variable to the **window** object:

````
```js
window.value = 123;
```

{window.value}
````

Even object keys get initialized, so for example doing:

```
{object.amount += 2}

{object.amount}
```

is going to display `2` to the reader!
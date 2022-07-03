
# The Language

Storytell comes with a simple markup language which is very similar to [markdown](https://www.markdownguide.org/). However, some features have been repurposed, some have been removed, and additional syntax has been added. If you're not familiar with markdown, I highly recommend going over the [basic sytnax](https://www.markdownguide.org/basic-syntax/). 

Here are the major differences:

| Markdown    |  Storytell |
| ----------- | ----------- |
| Headers (`#` and `##`)    | Path start |
| Lists (`- value`)         | Choices    |
| Code blocks               | Javascript code evaluation |

And here is everything storytell adds:

| Symbol  | Inline | Purpose |
|---------|--------|---------|
| `->`    | yes    | Diverting the reader to another part. |
| `<->`   | yes    | Diverting the reader to another part, and returning to the current path after the diverted path is finished. |
| `#[...]`| yes    | Attributes - changes the settings of other symbols. |
| `{ ... }` | yes  | Inline javascript code, which cannot contain curly brackets. | 
| `@{ ... }`| no   | Match a value without using javascript | 
| `// ...`  | no   | A piece of text which is meant for the author(s), it won't be displayed to the reader |
| `\`     | yes    | Ecapes a symbol. |
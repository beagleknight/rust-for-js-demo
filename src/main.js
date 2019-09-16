const fs = require("fs");
const someText = fs.readFileSync("file.txt").toString();
const ignoredWords = fs
  .readFileSync("stop_words.txt")
  .toString()
  .split("\n");

const wordCount = someText
  .toLowerCase()
  .split(/\s/)
  .reduce((total, word) => {
    if (ignoredWords.includes(word) || word == "" || word == "\n") {
      return total;
    }

    if (!total[word]) {
      total[word] = 0;
    }
    total[word] += 1;
    return total;
  }, {});

const result = Object.entries(wordCount)
  .sort((a, b) => b[1] - a[1])
  .slice(0, 5);

console.log(result);

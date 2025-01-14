import init, * as wasm from "./dist/site.js";

await init();

const examples = [
  {
    subheading: "Get the first three characters.",
    input: "abcd",
    grammar: "root = _{ char+ }\n\
char = { 'a'..'z' }",
    query: "char 1:3",
    exercises: [
      "Parse digits as well as characters and get the first three digits."
    ]
  }
];

CodeMirror.defineSimpleMode("pest", {
  start: [
    { regex: /\/\/.*/, token: "comment" },
    { regex: /[a-zA-Z_]\w*/, token: "constiable" },
    { regex: /=/, token: "operator", next: "mod" },
  ],
  mod: [
    { regex: /\{/, token: "bracket", next: "inside_rule" },
    { regex: /[_@!$]/, token: "operator-2" },
  ],
  inside_rule: [
    { regex: /\/\/.*/, token: "comment" },
    { regex: /"/, token: "string", next: "string" },
    {
      regex: /'(?:[^'\\]|\\(?:[nrt0'"]|x[\da-fA-F]{2}|u\{[\da-fA-F]{6}\}))'/,
      token: "string",
    },
    { regex: /\}/, token: "bracket", next: "start" },
    { regex: /#\w+/, token: "tag" },
    { regex: /[a-zA-Z_]\w*/, token: "constiable-2" },
    { regex: /=/, token: "operator-2" },
    { regex: /\d+/, token: "number" },
    { regex: /[~|*+?&!]|(\.\.)/, token: "operator" },
  ],
  string: [
    { regex: /"/, token: "string", next: "inside_rule" },
    { regex: /(?:[^\\"]|\\(?:.|$))*/, token: "string" },
  ],
  meta: {
    dontIndentStates: ["comment"],
    lineComment: "//",
  },
});

CodeMirror.defineSimpleMode("trql", {
  start: [
    { regex: /[a-z]+(?= *=)/, token: "name" },
    { regex: /[a-z]+/, token: "token" },
    { regex: /[\-:0-9]+/, token: "range" },
    { regex: /[\.…,\(\)\[\]]/, token: "other" }
  ]
});

CodeMirror.defineSimpleMode("yaml", {
  start: [
    { regex: /[a-z]+:/, token: "name" },
    { regex: /-/, token: "other" },
    { regex: /[a-z]+/, token: "value" },
    { regex: /./, token: "other" }
  ]
});

const input_editor = document.getElementById("input-textarea");
input_editor.addEventListener("input", update);

const grammar_editor = CodeMirror.fromTextArea(document.getElementById("grammar-textarea"), {
  mode: "pest",
  theme: "trql"
});
grammar_editor.getWrapperElement().id = "grammar-editor";
grammar_editor.on("change", update);

const query_editor = CodeMirror.fromTextArea(document.getElementById("query-textarea"), {
  mode: "trql",
  theme: "trql"
});
query_editor.getWrapperElement().id = "query-editor";
query_editor.on("change", update);

const output_editor = CodeMirror.fromTextArea(document.getElementById("output-textarea"), {
  mode: "yaml",
  theme: "trql",
  readOnly: true
});
const output_textarea = output_editor.getWrapperElement();
output_textarea.id = "output-editor";

function update() {
  output_editor.setValue(wasm.execute(
    input_editor.value,
    grammar_editor.getValue(),
    query_editor.getValue()
  ));
}

const example_title = document.getElementById("example-title");
const example_subheading = document.getElementById("example-subheading");
const example_exercises = document.getElementById("example-exercises");
const prev_example = document.getElementById("prev-example");
const next_example = document.getElementById("next-example");
const examples_len = examples.length;
let example_index = 0;

function update_example() {
  let example = examples[example_index];
  example_title.textContent = "Example " + (example_index + 1);
  example_subheading.textContent = example.subheading;
  example_exercises.textContent = "";
  example.exercises.forEach(item => {
    let li = document.createElement("li");
    li.textContent = item;
    example_exercises.appendChild(li);
  })
  input_editor.value = example.input;
  grammar_editor.setValue(example.grammar);
  query_editor.setValue(example.query);
}

prev_example.addEventListener('click', () => {
  example_index = (example_index + examples_len - 1) % examples_len;
  update_example();
});

next_example.addEventListener('click', () => {
  example_index = (example_index + 1) % examples_len;
  update_example();
});

update_example();


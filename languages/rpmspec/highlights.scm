(identifier) @variable
(special_variable_name) @constant

(macro_builtin) @variable.builtin

(macro_simple_expansion
  "%" @punctuation.special) @none
(macro_expansion
  "%{" @punctuation.special
  "}" @punctuation.special) @none
(macro_definition
  "%" @punctuation.special
  (macro_builtin) @keyword.directive.define
  (identifier) @keyword.macro)
(macro_undefinition
  (macro_builtin) @keyword.directive.define
  (identifier) @keyword.macro)

(macro_expansion
  (identifier) @function.call
  argument: [
    (word) @variable.parameter
    (concatenation
      (word) @variable.parameter)
  ])

(macro_call
  name: (macro_simple_expansion
          (identifier) @function.call))
(macro_call
  name: (macro_simple_expansion
          (identifier) @function.call)
  argument: [
    (word) @variable.parameter
    (concatenation
      (word) @variable.parameter)
  ])

[
  (tag)
  (dependency_tag)
] @type.definition

(integer) @number
(float) @number.float
(version) @number.float

(comment) @comment
;(string) @string
(quoted_string) @string

(description
  (section_name) @type.definition)
(package
  (section_name) @type.definition)
(files
  (section_name) @type.definition)
(changelog
  (section_name) @type.definition)

(prep_scriptlet
  (section_name) @function.builtin)
(generate_buildrequires
  (section_name) @function.builtin)
(conf_scriptlet
  (section_name) @function.builtin)
(build_scriptlet
  (section_name) @function.builtin)
(install_scriptlet
  (section_name) @function.builtin)
(check_scriptlet
  (section_name) @function.builtin)
(clean_scriptlet
  (section_name) @function.builtin)

[
  "%pre"
  "%post"
  "%preun"
  "%postun"
  "%pretrans"
  "%posttrans"
  "%preuntrans"
  "%postuntrans"
  "%verify"
] @function.builtin

[
  "%triggerprein"
  "%triggerin"
  "%triggerun"
  "%triggerpostun"
] @function.builtin

[
  "%filetriggerin"
  "%filetriggerun"
  "%filetriggerpostun"
  "%transfiletriggerin"
  "%transfiletriggerun"
  "%transfiletriggerpostun"
] @function.builtin

[
  "%artifact"
  "%attr"
  "%config"
  "%dir"
  "%doc"
  "%docdir"
  "%ghost"
  "%license"
  "%missingok"
  "%readme"
] @keyword.type

[
  "!="
  "<"
  "<="
  "=="
  ">"
  ">="
  "and"
  "&&"
  "or"
  "||"
] @operator

[
  "with"
  "without"
  "defined"
  "undefined"
] @keyword.operator

[
  "%if"
  "%ifarch"
  "%ifos"
  "%ifnarch"
  "%ifnos"
  "%elif"
  "%elifarch"
  "%elifos"
  "%else"
  "%endif"
] @keyword.conditional

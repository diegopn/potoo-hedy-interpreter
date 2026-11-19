//! Host-driven interpreter for the documented 16-level Hedy language profile.

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Level {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
}

impl TryFrom<u8> for Level {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            10 => Ok(Self::Ten),
            11 => Ok(Self::Eleven),
            12 => Ok(Self::Twelve),
            13 => Ok(Self::Thirteen),
            14 => Ok(Self::Fourteen),
            15 => Ok(Self::Fifteen),
            16 => Ok(Self::Sixteen),
            _ => Err("supported Hedy levels are 1 through 16".to_owned()),
        }
    }
}

impl Level {
    pub fn number(self) -> u8 {
        self as u8
    }
}

/// Command words are supplied separately from execution semantics so callers
/// can provide a verified localized vocabulary without changing the parser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vocabulary {
    pub print: String,
    pub ask: String,
    pub answer: String,
    pub echo: String,
    pub is: String,
    pub at: String,
    pub random: String,
    pub add: String,
    pub to: String,
    pub remove: String,
    pub from: String,
    pub sleep: String,
    pub forward: String,
    pub turn: String,
    pub left: String,
    pub right: String,
    pub play: String,
    pub clear: String,
    pub color: String,
    pub if_word: String,
    pub else_word: String,
    pub elif: String,
    pub and: String,
    pub or: String,
    pub not: String,
    pub in_word: String,
    pub repeat: String,
    pub times: String,
    pub for_word: String,
    pub range: String,
    pub define: String,
    pub call: String,
    pub with: String,
    pub pressed: String,
    pub return_word: String,
    pub while_word: String,
    pub input: String,
}

impl Vocabulary {
    pub fn english() -> Self {
        Self {
            print: "print".to_owned(),
            ask: "ask".to_owned(),
            answer: "answer".to_owned(),
            echo: "echo".to_owned(),
            is: "is".to_owned(),
            at: "at".to_owned(),
            random: "random".to_owned(),
            add: "add".to_owned(),
            to: "to".to_owned(),
            remove: "remove".to_owned(),
            from: "from".to_owned(),
            sleep: "sleep".to_owned(),
            forward: "forward".to_owned(),
            turn: "turn".to_owned(),
            left: "left".to_owned(),
            right: "right".to_owned(),
            play: "play".to_owned(),
            clear: "clear".to_owned(),
            color: "color".to_owned(),
            if_word: "if".to_owned(),
            else_word: "else".to_owned(),
            elif: "elif".to_owned(),
            and: "and".to_owned(),
            or: "or".to_owned(),
            not: "not".to_owned(),
            in_word: "in".to_owned(),
            repeat: "repeat".to_owned(),
            times: "times".to_owned(),
            for_word: "for".to_owned(),
            range: "range".to_owned(),
            define: "define".to_owned(),
            call: "call".to_owned(),
            with: "with".to_owned(),
            pressed: "pressed".to_owned(),
            return_word: "return".to_owned(),
            while_word: "while".to_owned(),
            input: "input".to_owned(),
        }
    }
}

impl Default for Vocabulary {
    fn default() -> Self {
        Self::english()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub span: Span,
    pub message: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.span.line, self.span.column, self.message
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Text(String),
    Number(f64),
    Boolean(bool),
    List(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(text) => f.write_str(text),
            Self::Number(number) => write!(f, "{number}"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::List(values) => {
                let rendered = values
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{rendered}]")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Program {
    level: Level,
    vocabulary: Vocabulary,
    statements: Vec<Statement>,
}

impl Program {
    pub fn level(&self) -> Level {
        self.level
    }
}

#[derive(Clone, Debug)]
struct Statement {
    span: Span,
    kind: StatementKind,
}

#[derive(Clone, Debug)]
enum StatementKind {
    Print(Vec<Part>),
    PrintExpressions(Vec<Expression>),
    Ask {
        target: Option<String>,
        prompt: Vec<Part>,
    },
    Assign {
        name: String,
        value: Assignment,
    },
    AddTo {
        item: Vec<Part>,
        list: String,
    },
    RemoveFrom {
        item: Vec<Part>,
        list: String,
    },
    Sleep(f64),
    SleepExpression(Expression),
    Forward(f64),
    ForwardExpression(Expression),
    Turn(Turn),
    TurnExpression(Expression),
    Play(String),
    PlayExpression(Expression),
    Clear,
    Color(Vec<Part>),
    ColorExpression(Expression),
    Echo(String),
    AssignExpression {
        name: String,
        expression: Expression,
    },
    If {
        branches: Vec<(Expression, Vec<Statement>)>,
        otherwise: Vec<Statement>,
    },
    IfPressed {
        key: String,
        body: Vec<Statement>,
        otherwise: Vec<Statement>,
    },
    Repeat {
        count: Expression,
        body: Vec<Statement>,
    },
    ForList {
        variable: String,
        list: String,
        body: Vec<Statement>,
    },
    ForRange {
        variable: String,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        end_inclusive: bool,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Define {
        name: String,
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Call {
        name: String,
        arguments: Vec<Expression>,
    },
    Return(Option<Expression>),
}

#[derive(Clone, Debug)]
enum Assignment {
    Parts(Vec<Part>),
    List(Vec<Vec<Part>>),
}

#[derive(Clone, Debug)]
enum Part {
    Template(String),
    Literal(String),
    Variable(String),
    Answer,
    RandomList(String),
}

#[derive(Clone, Debug)]
enum Expression {
    Literal(Value),
    List(Vec<Expression>),
    Concatenate(Vec<Expression>),
    Name(String),
    RandomList(Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Input(Vec<Expression>),
    Call(String, Vec<Expression>),
}

#[derive(Clone, Copy, Debug)]
enum UnaryOperator {
    Negate,
}

#[derive(Clone, Copy, Debug)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    In,
    NotIn,
}

#[derive(Clone, Copy, Debug)]
pub enum Turn {
    Left,
    Right,
    Degrees(f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeError {
    pub span: Span,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.span.line, self.span.column, self.message
        )
    }
}

/// Services used by language commands that depend on the surrounding app.
/// Drawing, sound, and random selection have failing defaults for hosts that
/// do not provide those capabilities.
pub trait Host {
    fn input(&mut self, prompt: &str) -> Result<String, String>;
    fn output(&mut self, text: &str) -> Result<(), String>;

    fn random_index(&mut self, _length: usize) -> Result<usize, String> {
        Err("random selection is not available in this host".to_owned())
    }

    fn sleep(&mut self, _seconds: f64) -> Result<(), String> {
        Err("sleep is not available in this host".to_owned())
    }

    fn turtle_forward(&mut self, _steps: f64) -> Result<(), String> {
        Err("turtle drawing is not available in this host".to_owned())
    }

    fn turtle_turn(&mut self, _turn: Turn) -> Result<(), String> {
        Err("turtle drawing is not available in this host".to_owned())
    }

    fn turtle_color(&mut self, _color: &str) -> Result<(), String> {
        Err("turtle drawing is not available in this host".to_owned())
    }

    fn turtle_clear(&mut self) -> Result<(), String> {
        Err("turtle drawing is not available in this host".to_owned())
    }

    fn play_note(&mut self, _note: &str) -> Result<(), String> {
        Err("audio playback is not available in this host".to_owned())
    }

    fn key_pressed(&mut self, _key: &str) -> Result<bool, String> {
        Err("keyboard input is not available in this host".to_owned())
    }
}

pub fn parse(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
) -> Result<Program, Vec<Diagnostic>> {
    if level >= Level::Five {
        return parse_advanced(source, level, vocabulary);
    }

    let mut statements = Vec::new();
    let mut diagnostics = Vec::new();

    for (line_index, raw_line) in source.lines().enumerate() {
        let line_number = line_index + 1;
        let without_comment = strip_comment(raw_line);
        let line = without_comment.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let span = Span {
            line: line_number,
            column: raw_line.len() - raw_line.trim_start().len() + 1,
        };
        match parse_statement(line, span, level, vocabulary) {
            Ok(statement) => statements.push(statement),
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }

    if diagnostics.is_empty() {
        Ok(Program {
            level,
            vocabulary: vocabulary.clone(),
            statements,
        })
    } else {
        Err(diagnostics)
    }
}

#[derive(Clone, Debug)]
struct SourceLine {
    text: String,
    indent: usize,
    span: Span,
}

fn parse_advanced(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
) -> Result<Program, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut lines = Vec::new();
    for (line_index, raw_line) in source.lines().enumerate() {
        let leading = raw_line
            .chars()
            .take_while(|c| *c == ' ' || *c == '\t')
            .count();
        let prefix = &raw_line[..raw_line
            .char_indices()
            .nth(leading)
            .map_or(raw_line.len(), |(index, _)| index)];
        if prefix.contains('\t') {
            diagnostics.push(Diagnostic {
                span: Span {
                    line: line_index + 1,
                    column: 1,
                },
                message: "tabs cannot be used for Hedy indentation".to_owned(),
            });
        }
        let without_comment = strip_comment(raw_line);
        let content = without_comment.trim();
        if content.is_empty() {
            continue;
        }
        let indent = without_comment.len() - without_comment.trim_start().len();
        lines.push(SourceLine {
            text: content.to_owned(),
            indent,
            span: Span {
                line: line_index + 1,
                column: indent + 1,
            },
        });
    }

    let mut cursor = 0;
    let statements = parse_block(
        &lines,
        &mut cursor,
        0,
        level,
        vocabulary,
        0,
        &mut diagnostics,
    );
    if cursor < lines.len() {
        diagnostics.push(Diagnostic {
            span: lines[cursor].span,
            message: "unexpected indentation".to_owned(),
        });
    }
    if diagnostics.is_empty() {
        Ok(Program {
            level,
            vocabulary: vocabulary.clone(),
            statements,
        })
    } else {
        Err(diagnostics)
    }
}

fn is_quote_open(character: char) -> bool {
    matches!(
        character,
        '\'' | '"' | '‘' | '“' | '„' | '”' | '«' | '《' | '「'
    )
}

fn is_quote_close(open: char, close: char) -> bool {
    match open {
        '\'' | '"' | '”' => open == close,
        '‘' => close == '’',
        '“' => close == '”',
        '„' => matches!(close, '“' | '”'),
        '«' => close == '»',
        '《' => close == '》',
        '「' => close == '」',
        _ => false,
    }
}

fn find_quote_close(source: &str, start: usize, open: char) -> Option<(usize, usize)> {
    let mut escaped = false;
    for (offset, character) in source[start..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if is_quote_close(open, character) {
            return Some((start + offset, character.len_utf8()));
        }
    }
    None
}

fn is_comma(character: char) -> bool {
    matches!(character, ',' | '،' | '，' | '、')
}

fn strip_comment(line: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;
    for (index, character) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' && quote.is_some() {
            escaped = true;
            continue;
        }
        match quote {
            Some(active) if is_quote_close(active, character) => quote = None,
            None if is_quote_open(character) => quote = Some(character),
            None if character == '#' => return &line[..index],
            _ => {}
        }
    }
    line
}

fn parse_block(
    lines: &[SourceLine],
    cursor: &mut usize,
    indent: usize,
    level: Level,
    vocabulary: &Vocabulary,
    nesting: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<Statement> {
    let mut statements = Vec::new();
    while *cursor < lines.len() {
        let line = &lines[*cursor];
        if line.indent < indent {
            break;
        }
        if line.indent > indent {
            diagnostics.push(Diagnostic {
                span: line.span,
                message: "unexpected indentation".to_owned(),
            });
            *cursor += 1;
            continue;
        }
        if is_clause_line(&line.text, vocabulary) {
            break;
        }

        let statement = if begins_command(&line.text, &vocabulary.if_word) {
            parse_if_block(
                lines,
                cursor,
                indent,
                level,
                vocabulary,
                nesting,
                diagnostics,
            )
        } else if begins_command(&line.text, &vocabulary.repeat) {
            parse_repeat_block(
                lines,
                cursor,
                indent,
                level,
                vocabulary,
                nesting,
                diagnostics,
            )
        } else if begins_command(&line.text, &vocabulary.for_word) {
            parse_for_block(
                lines,
                cursor,
                indent,
                level,
                vocabulary,
                nesting,
                diagnostics,
            )
        } else if begins_command(&line.text, &vocabulary.while_word) {
            parse_while_block(
                lines,
                cursor,
                indent,
                level,
                vocabulary,
                nesting,
                diagnostics,
            )
        } else if begins_command(&line.text, &vocabulary.define)
            || begins_command(&line.text, "def")
        {
            parse_define_block(lines, cursor, indent, level, vocabulary, diagnostics)
        } else {
            let source_line = line.clone();
            *cursor += 1;
            parse_advanced_statement(&source_line, level, vocabulary)
        };

        match statement {
            Ok(statement) => statements.push(statement),
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }
    statements
}

fn parse_advanced_statement(
    line: &SourceLine,
    level: Level,
    vocabulary: &Vocabulary,
) -> Result<Statement, Diagnostic> {
    let text = &line.text;
    let span = line.span;
    let invalid = |message: String| Diagnostic { span, message };

    if text == &vocabulary.return_word || text.starts_with(&format!("{} ", vocabulary.return_word))
    {
        if level < Level::Fifteen {
            return Err(invalid("return is introduced at Hedy level 15".to_owned()));
        }
        let value = text[vocabulary.return_word.len()..].trim();
        return Ok(Statement {
            span,
            kind: StatementKind::Return(if value.is_empty() {
                None
            } else {
                Some(parse_expression(value, level, vocabulary, span)?)
            }),
        });
    }

    if begins_command(text, &vocabulary.call) {
        if level != Level::Twelve {
            return Err(invalid(
                "define/call syntax is available only at Hedy level 12".to_owned(),
            ));
        }
        let rest = text[vocabulary.call.len()..].trim();
        let delimiter = format!(" {} ", vocabulary.with);
        if rest.contains(&delimiter) {
            return Err(invalid(
                "level 12 functions do not take arguments; arguments are introduced at level 15"
                    .to_owned(),
            ));
        }
        if !valid_identifier(rest) {
            return Err(invalid("call requires a function name".to_owned()));
        }
        return Ok(Statement {
            span,
            kind: StatementKind::Call {
                name: rest.to_owned(),
                arguments: Vec::new(),
            },
        });
    }

    if level >= Level::Thirteen {
        if let Some(arguments) = call_arguments(text, &vocabulary.print) {
            let expressions = parse_print_arguments(arguments, level, vocabulary, span)?;
            if expressions.is_empty() {
                return Err(invalid("print requires an expression".to_owned()));
            }
            return Ok(Statement {
                span,
                kind: StatementKind::PrintExpressions(expressions),
            });
        }
        if begins_command(text, &vocabulary.print) {
            return Err(invalid(
                "Python-style print calls require parentheses from Hedy level 13".to_owned(),
            ));
        }
    }

    if let Some(rest) = command_tail(text, &vocabulary.echo) {
        if level != Level::One {
            return Err(invalid("echo is available only at Hedy level 1".to_owned()));
        }
        return Ok(Statement {
            span,
            kind: StatementKind::Echo(rest.to_owned()),
        });
    }

    if level >= Level::Seven && level < Level::Thirteen {
        if let Some(rest) = command_tail(text, &vocabulary.print) {
            return Ok(Statement {
                span,
                kind: StatementKind::PrintExpressions(parse_legacy_print(
                    rest, level, vocabulary, span,
                )?),
            });
        }
    }

    if level >= Level::Seven {
        if let Some((name, right)) = split_assignment_operator(text, level, vocabulary) {
            if !valid_identifier(name) {
                return Err(invalid(format!("invalid variable name `{name}`")));
            }
            if level >= Level::Thirteen && command_tail(right, &vocabulary.ask).is_some() {
                return Err(invalid(
                    "the `ask` assignment form is unavailable from Hedy level 13; use `input(...)`"
                        .to_owned(),
                ));
            }
            if let Some(prompt) = command_tail(right, &vocabulary.ask) {
                if prompt.is_empty() {
                    return Err(invalid("ask requires a question".to_owned()));
                }
                return Ok(Statement {
                    span,
                    kind: StatementKind::Ask {
                        target: Some(name.to_owned()),
                        prompt: parse_parts(prompt, level, vocabulary, span)?,
                    },
                });
            }
            let expression = if level >= Level::Thirteen {
                parse_expression(right, level, vocabulary, span)?
            } else {
                parse_legacy_value(right, level, vocabulary, span)?
            };
            return Ok(Statement {
                span,
                kind: StatementKind::AssignExpression {
                    name: name.to_owned(),
                    expression,
                },
            });
        }
    }

    if level >= Level::Fifteen {
        if let Some(open) = text.find('(') {
            if text.ends_with(')') {
                let name = text[..open].trim();
                if valid_identifier(name) {
                    let arguments = split_top_level(&text[open + 1..text.len() - 1], ',')
                        .into_iter()
                        .filter(|argument| !argument.trim().is_empty())
                        .map(|argument| parse_expression(argument.trim(), level, vocabulary, span))
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(Statement {
                        span,
                        kind: StatementKind::Call {
                            name: name.to_owned(),
                            arguments,
                        },
                    });
                }
            }
        }
    }

    if level >= Level::Thirteen && begins_command(text, &vocabulary.input) {
        return Err(invalid("input must be assigned to a variable".to_owned()));
    }

    parse_statement(text, span, level, vocabulary)
}

fn begins_command(line: &str, command: &str) -> bool {
    line.strip_prefix(command).is_some_and(|rest| {
        rest.is_empty() || rest.starts_with(char::is_whitespace) || rest.starts_with(':')
    })
}

fn is_clause_line(line: &str, vocabulary: &Vocabulary) -> bool {
    begins_command(line, &vocabulary.else_word) || begins_command(line, &vocabulary.elif)
}

fn child_indent(lines: &[SourceLine], cursor: usize, parent_indent: usize) -> Option<usize> {
    lines
        .get(cursor)
        .filter(|line| line.indent > parent_indent)
        .map(|line| line.indent)
}

fn take_body(
    lines: &[SourceLine],
    cursor: &mut usize,
    parent_indent: usize,
    level: Level,
    vocabulary: &Vocabulary,
    nesting: usize,
    diagnostics: &mut Vec<Diagnostic>,
    span: Span,
) -> Result<Vec<Statement>, Diagnostic> {
    let indent = child_indent(lines, *cursor, parent_indent).ok_or_else(|| Diagnostic {
        span,
        message: "expected an indented block".to_owned(),
    })?;
    Ok(parse_block(
        lines,
        cursor,
        indent,
        level,
        vocabulary,
        nesting + 1,
        diagnostics,
    ))
}

fn parse_if_block(
    lines: &[SourceLine],
    cursor: &mut usize,
    indent: usize,
    level: Level,
    vocabulary: &Vocabulary,
    nesting: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Statement, Diagnostic> {
    let line = lines[*cursor].clone();
    *cursor += 1;
    let mut remainder = line.text[vocabulary.if_word.len()..].trim();
    if level >= Level::Thirteen {
        if !remainder.ends_with(':') {
            return Err(Diagnostic {
                span: line.span,
                message: "Python-style if statements require a trailing colon".to_owned(),
            });
        }
        remainder = remainder[..remainder.len() - 1].trim_end();
    }
    if level >= Level::Thirteen {
        let condition_text = remainder.trim();
        let pressed_suffix = format!(" {} {}", vocabulary.is, vocabulary.pressed);
        if let Some(key) = condition_text.strip_suffix(&pressed_suffix) {
            let key = key.trim();
            if !valid_identifier(key) {
                return Err(Diagnostic {
                    span: line.span,
                    message: "pressed-key conditions require a key name".to_owned(),
                });
            }
            let body = take_body(
                lines,
                cursor,
                indent,
                level,
                vocabulary,
                nesting,
                diagnostics,
                line.span,
            )?;
            let otherwise = if let Some(clause) = lines
                .get(*cursor)
                .filter(|clause| {
                    clause.indent == indent && begins_command(&clause.text, &vocabulary.else_word)
                })
                .cloned()
            {
                *cursor += 1;
                let tail = clause.text[vocabulary.else_word.len()..].trim();
                if !tail.is_empty() && tail != ":" {
                    return Err(Diagnostic {
                        span: clause.span,
                        message: "pressed-key else branches use an indented block".to_owned(),
                    });
                }
                take_body(
                    lines,
                    cursor,
                    indent,
                    level,
                    vocabulary,
                    nesting,
                    diagnostics,
                    clause.span,
                )?
            } else {
                Vec::new()
            };
            return Ok(Statement {
                span: line.span,
                kind: StatementKind::IfPressed {
                    key: key.to_owned(),
                    body,
                    otherwise,
                },
            });
        }
    }
    if level <= Level::Eight {
        let inline = (level == Level::Five)
            .then(|| split_inline_if(remainder, level, vocabulary, line.span))
            .flatten();
        let (condition_text, body_text, inline_else) = if let Some(inline) = inline {
            (inline.0, Some(inline.1), inline.2)
        } else {
            (remainder.trim(), None, None)
        };
        let condition = parse_condition(condition_text, level, vocabulary, line.span)?;
        let body = take_simple_body(
            lines,
            cursor,
            indent,
            body_text,
            line.span,
            level,
            vocabulary,
            nesting,
            diagnostics,
        )?;
        let mut branches = vec![(condition, body)];
        let mut otherwise = if let Some(text) = inline_else {
            vec![parse_inline_simple(
                text,
                line.span,
                indent,
                level,
                vocabulary,
                nesting,
                diagnostics,
            )?]
        } else {
            Vec::new()
        };
        while inline_else.is_none() {
            let Some(clause) = lines.get(*cursor).filter(|clause| clause.indent == indent) else {
                break;
            };
            if begins_command(&clause.text, &vocabulary.elif) && level >= Level::Six {
                let clause = clause.clone();
                *cursor += 1;
                let clause_text = clause.text[vocabulary.elif.len()..].trim();
                let condition = parse_condition(clause_text, level, vocabulary, clause.span)?;
                let body = take_simple_body(
                    lines,
                    cursor,
                    indent,
                    None,
                    clause.span,
                    level,
                    vocabulary,
                    nesting,
                    diagnostics,
                )?;
                branches.push((condition, body));
            } else if begins_command(&clause.text, &vocabulary.else_word) {
                let clause = clause.clone();
                *cursor += 1;
                let tail = clause.text[vocabulary.else_word.len()..].trim();
                otherwise = if !tail.is_empty() {
                    if level != Level::Five {
                        return Err(Diagnostic {
                            span: clause.span,
                            message: "else commands must be on the next line from level 6"
                                .to_owned(),
                        });
                    }
                    vec![parse_inline_simple(
                        tail,
                        clause.span,
                        indent,
                        level,
                        vocabulary,
                        nesting,
                        diagnostics,
                    )?]
                } else {
                    take_simple_body(
                        lines,
                        cursor,
                        indent,
                        None,
                        clause.span,
                        level,
                        vocabulary,
                        nesting,
                        diagnostics,
                    )?
                };
                break;
            } else {
                break;
            }
        }
        return Ok(Statement {
            span: line.span,
            kind: StatementKind::If {
                branches,
                otherwise,
            },
        });
    }
    if remainder.ends_with(':') {
        if level < Level::Thirteen {
            return Err(Diagnostic {
                span: line.span,
                message: "colons are introduced at Hedy level 13".to_owned(),
            });
        }
        remainder = remainder[..remainder.len() - 1].trim_end();
    }
    if nesting > 0 && level < Level::Ten {
        return Err(Diagnostic {
            span: line.span,
            message: "nested blocks are introduced at Hedy level 10".to_owned(),
        });
    }
    let condition = parse_condition(remainder, level, vocabulary, line.span)?;
    let body = take_body(
        lines,
        cursor,
        indent,
        level,
        vocabulary,
        nesting,
        diagnostics,
        line.span,
    )?;
    let mut branches = vec![(condition, body)];
    let mut otherwise = Vec::new();
    while let Some(clause) = lines.get(*cursor).filter(|clause| clause.indent == indent) {
        if begins_command(&clause.text, &vocabulary.elif) {
            if level < Level::Six {
                return Err(Diagnostic {
                    span: clause.span,
                    message: "elif is introduced at Hedy level 6".to_owned(),
                });
            }
            let clause = clause.clone();
            *cursor += 1;
            let mut condition_text = clause.text[vocabulary.elif.len()..].trim();
            if level >= Level::Thirteen && !condition_text.ends_with(':') {
                return Err(Diagnostic {
                    span: clause.span,
                    message: "Python-style elif clauses require a trailing colon".to_owned(),
                });
            }
            if condition_text.ends_with(':') && level >= Level::Thirteen {
                condition_text = condition_text[..condition_text.len() - 1].trim_end();
            }
            let condition = parse_condition(condition_text, level, vocabulary, clause.span)?;
            let body = take_body(
                lines,
                cursor,
                indent,
                level,
                vocabulary,
                nesting,
                diagnostics,
                clause.span,
            )?;
            branches.push((condition, body));
        } else if begins_command(&clause.text, &vocabulary.else_word) {
            let clause = clause.clone();
            *cursor += 1;
            let tail = clause.text[vocabulary.else_word.len()..].trim();
            if level >= Level::Thirteen && tail != ":" {
                return Err(Diagnostic {
                    span: clause.span,
                    message: "Python-style else clauses require a trailing colon".to_owned(),
                });
            }
            if !tail.is_empty() && !tail.starts_with(':') {
                if level != Level::Five {
                    return Err(Diagnostic {
                        span: clause.span,
                        message: "else must start an indented block".to_owned(),
                    });
                }
                let tail = tail.strip_prefix(':').unwrap_or(tail).trim();
                let body_line = SourceLine {
                    text: tail.to_owned(),
                    indent,
                    span: clause.span,
                };
                otherwise = vec![parse_advanced_statement(&body_line, level, vocabulary)?];
            } else {
                if tail.starts_with(':') && level < Level::Thirteen {
                    return Err(Diagnostic {
                        span: clause.span,
                        message: "colons are introduced at Hedy level 13".to_owned(),
                    });
                }
                otherwise = take_body(
                    lines,
                    cursor,
                    indent,
                    level,
                    vocabulary,
                    nesting,
                    diagnostics,
                    clause.span,
                )?;
            }
            break;
        } else {
            break;
        }
    }
    Ok(Statement {
        span: line.span,
        kind: StatementKind::If {
            branches,
            otherwise,
        },
    })
}

fn parse_repeat_block(
    lines: &[SourceLine],
    cursor: &mut usize,
    indent: usize,
    level: Level,
    vocabulary: &Vocabulary,
    nesting: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Statement, Diagnostic> {
    let line = lines[*cursor].clone();
    *cursor += 1;
    if !(Level::Eight..Level::Thirteen).contains(&level) {
        return Err(Diagnostic {
            span: line.span,
            message: "repeat syntax is available from Hedy levels 8 through 12".to_owned(),
        });
    }
    let rest = line.text[vocabulary.repeat.len()..].trim();
    let times = format!(" {}", vocabulary.times);
    if let Some((count, inline)) = rest.split_once(&times) {
        let count = parse_legacy_value(count.trim(), level, vocabulary, line.span)?;
        if !inline.trim().is_empty() {
            if level != Level::Eight {
                return Err(Diagnostic {
                    span: line.span,
                    message: "inline repeat commands are available only at Hedy level 8".to_owned(),
                });
            }
            let child = SourceLine {
                text: inline.trim().to_owned(),
                indent,
                span: line.span,
            };
            let body = vec![parse_advanced_statement(&child, level, vocabulary)?];
            return Ok(Statement {
                span: line.span,
                kind: StatementKind::Repeat { count, body },
            });
        }
        if level < Level::Nine {
            return Err(Diagnostic {
                span: line.span,
                message: "repeating multiple lines is introduced at Hedy level 9".to_owned(),
            });
        }
        if nesting > 0 && level < Level::Ten {
            return Err(Diagnostic {
                span: line.span,
                message: "nested blocks are introduced at Hedy level 10".to_owned(),
            });
        }
        let body = take_body(
            lines,
            cursor,
            indent,
            level,
            vocabulary,
            nesting,
            diagnostics,
            line.span,
        )?;
        return Ok(Statement {
            span: line.span,
            kind: StatementKind::Repeat { count, body },
        });
    }
    Err(Diagnostic {
        span: line.span,
        message: "repeat syntax requires a count and `times`".to_owned(),
    })
}

fn parse_for_block(
    lines: &[SourceLine],
    cursor: &mut usize,
    indent: usize,
    level: Level,
    vocabulary: &Vocabulary,
    nesting: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Statement, Diagnostic> {
    let line = lines[*cursor].clone();
    *cursor += 1;
    if level < Level::Eleven || level == Level::Thirteen {
        return Err(Diagnostic {
            span: line.span,
            message: "for loops are available at levels 11–12 and 14–16".to_owned(),
        });
    }
    if level >= Level::Fourteen && !line.text.ends_with(':') {
        return Err(Diagnostic {
            span: line.span,
            message: "Python-style for loops require a trailing colon".to_owned(),
        });
    }
    let body = take_body(
        lines,
        cursor,
        indent,
        level,
        vocabulary,
        nesting,
        diagnostics,
        line.span,
    )?;
    let rest = line.text[vocabulary.for_word.len()..].trim();
    if let Some((variable, source)) = rest.split_once(&format!(" {} ", vocabulary.in_word)) {
        let variable = variable.trim();
        if !valid_identifier(variable) {
            return Err(Diagnostic {
                span: line.span,
                message: "for loop requires a variable name".to_owned(),
            });
        }
        let source = source.trim().trim_end_matches(':').trim();
        if level >= Level::Fourteen && source.starts_with(&vocabulary.range) {
            if let Some(arguments) = call_arguments(source, &vocabulary.range) {
                let args = split_top_level(arguments, ',');
                if args.len() != 2 {
                    return Err(Diagnostic {
                        span: line.span,
                        message: "range requires a start and stop value".to_owned(),
                    });
                }
                let start = parse_expression(args[0].trim(), level, vocabulary, line.span)?;
                let end = parse_expression(args[1].trim(), level, vocabulary, line.span)?;
                let step = None;
                let end_inclusive = false;
                return Ok(Statement {
                    span: line.span,
                    kind: StatementKind::ForRange {
                        variable: variable.to_owned(),
                        start,
                        end,
                        step,
                        end_inclusive,
                        body,
                    },
                });
            }
        }
        if valid_identifier(source) {
            return Ok(Statement {
                span: line.span,
                kind: StatementKind::ForList {
                    variable: variable.to_owned(),
                    list: source.to_owned(),
                    body,
                },
            });
        }
    }
    Err(Diagnostic {
        span: line.span,
        message: "for syntax requires `variable in list` or a documented range".to_owned(),
    })
}

fn parse_while_block(
    lines: &[SourceLine],
    cursor: &mut usize,
    indent: usize,
    level: Level,
    vocabulary: &Vocabulary,
    nesting: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Statement, Diagnostic> {
    let line = lines[*cursor].clone();
    *cursor += 1;
    if level < Level::Sixteen {
        return Err(Diagnostic {
            span: line.span,
            message: "while is introduced at Hedy level 16".to_owned(),
        });
    }
    let mut condition_text = line.text[vocabulary.while_word.len()..].trim();
    if condition_text.ends_with(':') {
        condition_text = condition_text[..condition_text.len() - 1].trim_end();
    }
    let condition = parse_condition(condition_text, level, vocabulary, line.span)?;
    let body = take_body(
        lines,
        cursor,
        indent,
        level,
        vocabulary,
        nesting,
        diagnostics,
        line.span,
    )?;
    Ok(Statement {
        span: line.span,
        kind: StatementKind::While { condition, body },
    })
}

fn parse_define_block(
    lines: &[SourceLine],
    cursor: &mut usize,
    indent: usize,
    level: Level,
    vocabulary: &Vocabulary,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Statement, Diagnostic> {
    let line = lines[*cursor].clone();
    *cursor += 1;
    if level < Level::Twelve {
        return Err(Diagnostic {
            span: line.span,
            message: "functions are introduced at Hedy level 12".to_owned(),
        });
    }
    let python_syntax = begins_command(&line.text, "def");
    let rest = if python_syntax {
        if level < Level::Fifteen {
            return Err(Diagnostic {
                span: line.span,
                message: "Python-style function syntax is introduced at Hedy level 15".to_owned(),
            });
        }
        if !line.text.ends_with(':') {
            return Err(Diagnostic {
                span: line.span,
                message: "Python-style function definitions require a trailing colon".to_owned(),
            });
        }
        &line.text[3..]
    } else {
        if level != Level::Twelve {
            return Err(Diagnostic {
                span: line.span,
                message: "define syntax is available only at Hedy level 12".to_owned(),
            });
        }
        &line.text[vocabulary.define.len()..]
    };
    let rest = rest.trim().trim_end_matches(':').trim();
    let (name, parameters) = if let Some(open) = rest.find('(') {
        if !python_syntax {
            return Err(Diagnostic {
                span: line.span,
                message: "level 12 functions do not take parameters".to_owned(),
            });
        }
        let name = rest[..open].trim();
        let close = rest.rfind(')').ok_or_else(|| Diagnostic {
            span: line.span,
            message: "missing `)` in function definition".to_owned(),
        })?;
        let parameters = split_top_level(&rest[open + 1..close], ',')
            .into_iter()
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        (name, parameters)
    } else if !python_syntax {
        let delimiter = format!(" {} ", vocabulary.with);
        if rest.contains(&delimiter) {
            return Err(Diagnostic {
                span: line.span,
                message:
                    "level 12 functions do not take parameters; parameters are introduced at level 15"
                        .to_owned(),
            });
        }
        (rest, Vec::new())
    } else {
        if level >= Level::Fifteen && rest.contains(' ') {
            return Err(Diagnostic {
                span: line.span,
                message: "function parameters must be enclosed in parentheses".to_owned(),
            });
        }
        (rest, Vec::new())
    };
    if !valid_identifier(name)
        || parameters
            .iter()
            .any(|parameter| !valid_identifier(parameter))
    {
        return Err(Diagnostic {
            span: line.span,
            message: "invalid function or parameter name".to_owned(),
        });
    }
    let body = take_body(
        lines,
        cursor,
        indent,
        level,
        vocabulary,
        0,
        diagnostics,
        line.span,
    )?;
    Ok(Statement {
        span: line.span,
        kind: StatementKind::Define {
            name: name.to_owned(),
            parameters,
            body,
        },
    })
}

fn split_inline_if<'a>(
    source: &'a str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Option<(&'a str, &'a str, Option<&'a str>)> {
    let (before_else, else_body) = split_once_outside_quotes(source, " else ")
        .map_or((source, None), |(before, after)| {
            (before, Some(after.trim()))
        });
    for (index, character) in before_else.char_indices() {
        if !character.is_whitespace() {
            continue;
        }
        let condition = before_else[..index].trim();
        let command = before_else[index..].trim();
        if condition.is_empty() || command.is_empty() {
            continue;
        }
        if parse_condition(condition, level, vocabulary, span).is_err() {
            continue;
        }
        if begins_command(command, &vocabulary.repeat) && level == Level::Eight {
            return Some((condition, command, else_body));
        }
        if parse_advanced_statement(
            &SourceLine {
                text: command.to_owned(),
                indent: 0,
                span,
            },
            level,
            vocabulary,
        )
        .is_ok()
        {
            return Some((condition, command, else_body));
        }
    }
    None
}

fn split_once_outside_quotes<'a>(source: &'a str, delimiter: &str) -> Option<(&'a str, &'a str)> {
    let mut quote = None;
    let mut escaped = false;
    for (index, character) in source.char_indices() {
        if quote.is_none() && source[index..].starts_with(delimiter) {
            return Some((&source[..index], &source[index + delimiter.len()..]));
        }
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' && quote.is_some() {
            escaped = true;
            continue;
        }
        match quote {
            Some(active) if is_quote_close(active, character) => quote = None,
            None if is_quote_open(character) => quote = Some(character),
            _ => {}
        }
    }
    None
}

fn take_simple_body(
    lines: &[SourceLine],
    cursor: &mut usize,
    indent: usize,
    inline: Option<&str>,
    span: Span,
    level: Level,
    vocabulary: &Vocabulary,
    nesting: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Vec<Statement>, Diagnostic> {
    if let Some(text) = inline {
        return Ok(vec![parse_inline_simple(
            text,
            span,
            indent,
            level,
            vocabulary,
            nesting,
            diagnostics,
        )?]);
    }
    let line = lines
        .get(*cursor)
        .filter(|line| line.indent == indent && !is_clause_line(&line.text, vocabulary))
        .ok_or_else(|| Diagnostic {
            span,
            message: "expected one command after the conditional".to_owned(),
        })?
        .clone();
    *cursor += 1;
    Ok(vec![parse_inline_simple(
        &line.text,
        line.span,
        line.indent,
        level,
        vocabulary,
        nesting,
        diagnostics,
    )?])
}

fn parse_inline_simple(
    text: &str,
    span: Span,
    indent: usize,
    level: Level,
    vocabulary: &Vocabulary,
    nesting: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Statement, Diagnostic> {
    if begins_command(text, &vocabulary.repeat) {
        let lines = [SourceLine {
            text: text.to_owned(),
            indent,
            span,
        }];
        let mut cursor = 0;
        parse_repeat_block(
            &lines,
            &mut cursor,
            indent,
            level,
            vocabulary,
            nesting,
            diagnostics,
        )
    } else {
        parse_advanced_statement(
            &SourceLine {
                text: text.to_owned(),
                indent,
                span,
            },
            level,
            vocabulary,
        )
    }
}

fn split_assignment_operator<'a>(
    line: &'a str,
    level: Level,
    vocabulary: &Vocabulary,
) -> Option<(&'a str, &'a str)> {
    if level >= Level::Seven {
        for (index, character) in line.char_indices() {
            if character == '='
                && line.as_bytes().get(index.wrapping_sub(1)) != Some(&b'=')
                && line.as_bytes().get(index + 1) != Some(&b'=')
            {
                let left = line[..index].trim();
                let right = line[index + 1..].trim();
                if !left.is_empty() && !right.is_empty() {
                    return Some((left, right));
                }
            }
        }
        let delimiter = format!(" {} ", vocabulary.is);
        if let Some((left, right)) = line.split_once(&delimiter) {
            if !left.trim().is_empty() && !right.trim().is_empty() {
                return Some((left.trim(), right.trim()));
            }
        }
        return None;
    }
    let delimiter = format!(" {} ", vocabulary.is);
    let (left, right) = line.split_once(&delimiter)?;
    Some((left.trim(), right.trim()))
}

fn call_arguments<'a>(line: &'a str, command: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(command)?.trim_start();
    let rest = rest.strip_prefix('(')?;
    let close = rest.rfind(')')?;
    if !rest[close + 1..].trim().is_empty() {
        return None;
    }
    Some(&rest[..close])
}

fn parse_print_arguments(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Result<Vec<Expression>, Diagnostic> {
    if source.trim().is_empty() {
        return Err(Diagnostic {
            span,
            message: "print requires an expression".to_owned(),
        });
    }
    let arguments = split_top_level(source, ',');
    if arguments.len() > 1 {
        return arguments
            .into_iter()
            .filter(|argument| !argument.trim().is_empty())
            .map(|argument| parse_output_expression(argument.trim(), level, vocabulary, span))
            .collect();
    }
    Ok(vec![parse_output_expression(
        source.trim(),
        level,
        vocabulary,
        span,
    )?])
}

fn parse_legacy_print(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Result<Vec<Expression>, Diagnostic> {
    let arguments = split_top_level(source, ',');
    if arguments.len() > 1 {
        return arguments
            .into_iter()
            .filter(|argument| !argument.trim().is_empty())
            .map(|argument| parse_output_expression(argument.trim(), level, vocabulary, span))
            .collect();
    }
    match parse_expression(source.trim(), level, vocabulary, span) {
        Ok(expression) => Ok(vec![expression]),
        Err(_) => parse_interpolated_expressions(source, level, vocabulary, span),
    }
}

fn parse_output_expression(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Result<Expression, Diagnostic> {
    match parse_expression(source, level, vocabulary, span) {
        Ok(expression) => Ok(expression),
        Err(_) => parse_interpolated_expressions(source, level, vocabulary, span)
            .map(Expression::Concatenate),
    }
}

fn parse_interpolated_expressions(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Result<Vec<Expression>, Diagnostic> {
    let mut parts = Vec::new();
    let mut cursor = 0;
    while cursor < source.len() {
        while cursor < source.len() {
            let character = source[cursor..]
                .chars()
                .next()
                .expect("cursor is in bounds");
            if !character.is_whitespace() {
                break;
            }
            cursor += character.len_utf8();
        }
        if cursor == source.len() {
            break;
        }
        let character = source[cursor..]
            .chars()
            .next()
            .expect("cursor is in bounds");
        if is_quote_open(character) {
            let quote = character;
            let start = cursor + quote.len_utf8();
            let (close, close_length) =
                find_quote_close(source, start, quote).ok_or_else(|| Diagnostic {
                    span,
                    message: "unterminated quoted text".to_owned(),
                })?;
            let text = unescape_string(&source[start..close]);
            parts.push(Expression::Literal(Value::Text(text)));
            cursor = close + close_length;
        } else {
            let start = cursor;
            let mut quote = None;
            let mut depth = 0usize;
            let mut escaped = false;
            let mut end = source.len();
            for (offset, current) in source[cursor..].char_indices() {
                if escaped {
                    escaped = false;
                    continue;
                }
                if current == '\\' && quote.is_some() {
                    escaped = true;
                    continue;
                }
                match quote {
                    Some(active) if is_quote_close(active, current) => quote = None,
                    None if is_quote_open(current) && depth == 0 => {
                        end = cursor + offset;
                        break;
                    }
                    None if is_quote_open(current) => quote = Some(current),
                    None if matches!(current, '(' | '[') => depth += 1,
                    None if matches!(current, ')' | ']') => depth = depth.saturating_sub(1),
                    _ => {}
                }
            }
            let expression_text = source[start..end].trim();
            if expression_text.is_empty() {
                return Err(Diagnostic {
                    span,
                    message: "expected an expression after quoted text".to_owned(),
                });
            }
            parts.push(parse_expression(expression_text, level, vocabulary, span)?);
            cursor = end;
        }
    }
    Ok(parts)
}

fn unescape_string(value: &str) -> String {
    let mut result = String::new();
    let mut characters = value.chars();
    while let Some(character) = characters.next() {
        if character == '\\' {
            match characters.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some(escaped) => result.push(escaped),
                None => result.push('\\'),
            }
        } else {
            result.push(character);
        }
    }
    result
}

fn split_top_level(source: &str, separator: char) -> Vec<&str> {
    let mut pieces = Vec::new();
    let mut start = 0;
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;
    for (index, character) in source.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' && quote.is_some() {
            escaped = true;
            continue;
        }
        match quote {
            Some(active) if is_quote_close(active, character) => quote = None,
            None if is_quote_open(character) => quote = Some(character),
            None if matches!(character, '(' | '[') => depth += 1,
            None if matches!(character, ')' | ']') => depth = depth.saturating_sub(1),
            None if (character == separator || (separator == ',' && is_comma(character)))
                && depth == 0 =>
            {
                pieces.push(&source[start..index]);
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }
    pieces.push(&source[start..]);
    pieces
}

#[derive(Clone, Debug, PartialEq)]
enum ExprToken {
    Number(f64),
    Word(String),
    Text(String),
    Operator(String),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
}

fn lex_expression(source: &str, span: Span) -> Result<Vec<ExprToken>, Diagnostic> {
    let mut tokens = Vec::new();
    let mut cursor = 0;
    while cursor < source.len() {
        let character = source[cursor..]
            .chars()
            .next()
            .expect("cursor is on a UTF-8 boundary");
        if character.is_whitespace() {
            cursor += character.len_utf8();
            continue;
        }
        if is_quote_open(character) {
            let quote = character;
            cursor += quote.len_utf8();
            let mut value = String::new();
            let mut closed = false;
            while cursor < source.len() {
                let current = source[cursor..]
                    .chars()
                    .next()
                    .expect("cursor is on a UTF-8 boundary");
                cursor += current.len_utf8();
                if is_quote_close(quote, current) {
                    closed = true;
                    break;
                }
                if current == '\\' && cursor < source.len() {
                    let escaped = source[cursor..]
                        .chars()
                        .next()
                        .expect("cursor is on a UTF-8 boundary");
                    value.push(escaped);
                    cursor += escaped.len_utf8();
                } else {
                    value.push(current);
                }
            }
            if !closed {
                return Err(Diagnostic {
                    span,
                    message: "unterminated string literal".to_owned(),
                });
            }
            tokens.push(ExprToken::Text(value));
            continue;
        }
        if character.is_ascii_digit() {
            let start = cursor;
            cursor += character.len_utf8();
            while cursor < source.len() {
                let current = source[cursor..]
                    .chars()
                    .next()
                    .expect("cursor is on a UTF-8 boundary");
                if current.is_ascii_digit() || current == '.' {
                    cursor += current.len_utf8();
                } else {
                    break;
                }
            }
            let number = source[start..cursor]
                .parse::<f64>()
                .map_err(|_| Diagnostic {
                    span,
                    message: format!("invalid number `{}`", &source[start..cursor]),
                })?;
            if !number.is_finite() {
                return Err(Diagnostic {
                    span,
                    message: "number must be finite".to_owned(),
                });
            }
            tokens.push(ExprToken::Number(number));
            continue;
        }
        if character.is_alphabetic() || character == '_' {
            let start = cursor;
            cursor += character.len_utf8();
            while cursor < source.len() {
                let current = source[cursor..]
                    .chars()
                    .next()
                    .expect("cursor is on a UTF-8 boundary");
                if current.is_alphanumeric() || current == '_' {
                    cursor += current.len_utf8();
                } else {
                    break;
                }
            }
            tokens.push(ExprToken::Word(source[start..cursor].to_owned()));
            continue;
        }
        let rest = &source[cursor..];
        let (token, length) = if let Some((operator, length)) = ["==", "!=", "<=", ">="]
            .iter()
            .find_map(|op| rest.starts_with(op).then_some((*op, op.len())))
        {
            (ExprToken::Operator(operator.to_owned()), length)
        } else {
            match character {
                '(' => (ExprToken::LeftParen, 1),
                ')' => (ExprToken::RightParen, 1),
                '[' => (ExprToken::LeftBracket, 1),
                ']' => (ExprToken::RightBracket, 1),
                ',' | '،' | '，' | '、' => (ExprToken::Comma, character.len_utf8()),
                '+' | '-' | '*' | '/' | '<' | '>' | '=' => {
                    (ExprToken::Operator(character.to_string()), 1)
                }
                _ => {
                    return Err(Diagnostic {
                        span,
                        message: format!("unexpected character `{character}` in expression"),
                    });
                }
            }
        };
        tokens.push(token);
        cursor += length;
    }
    Ok(tokens)
}

struct ExpressionParser<'a> {
    tokens: Vec<ExprToken>,
    cursor: usize,
    level: Level,
    vocabulary: &'a Vocabulary,
    span: Span,
}

impl ExpressionParser<'_> {
    fn parse(&mut self) -> Result<Expression, Diagnostic> {
        let expression = self.parse_binary(0)?;
        if self.cursor != self.tokens.len() {
            return Err(Diagnostic {
                span: self.span,
                message: "unexpected token after expression".to_owned(),
            });
        }
        Ok(expression)
    }

    fn parse_binary(&mut self, minimum_precedence: u8) -> Result<Expression, Diagnostic> {
        let mut left = self.parse_unary()?;
        loop {
            let Some((operator, precedence, consumed)) = self.binary_operator() else {
                break;
            };
            if precedence < minimum_precedence {
                break;
            }
            self.cursor += consumed;
            let right = self.parse_binary(precedence + 1)?;
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }
        Ok(left)
    }

    fn binary_operator(&self) -> Option<(BinaryOperator, u8, usize)> {
        match self.tokens.get(self.cursor)? {
            ExprToken::Operator(op) => {
                let required_level = match op.as_str() {
                    "==" | "!=" | "<" | "<=" | ">" | ">=" => Level::Thirteen,
                    "=" => Level::Seven,
                    "+" | "-" | "*" | "/" => Level::Seven,
                    _ => return None,
                };
                if self.level < required_level {
                    return None;
                }
                Some(match op.as_str() {
                    "==" | "=" => (BinaryOperator::Equal, 3, 1),
                    "!=" => (BinaryOperator::NotEqual, 3, 1),
                    "<" => (BinaryOperator::Less, 3, 1),
                    "<=" => (BinaryOperator::LessEqual, 3, 1),
                    ">" => (BinaryOperator::Greater, 3, 1),
                    ">=" => (BinaryOperator::GreaterEqual, 3, 1),
                    "+" => (BinaryOperator::Add, 4, 1),
                    "-" => (BinaryOperator::Subtract, 4, 1),
                    "*" => (BinaryOperator::Multiply, 5, 1),
                    "/" => (BinaryOperator::Divide, 5, 1),
                    _ => return None,
                })
            }
            ExprToken::Word(word) if word == &self.vocabulary.is => {
                Some((BinaryOperator::Equal, 3, 1))
            }
            ExprToken::Word(word) if word == &self.vocabulary.and && self.level >= Level::Ten => {
                Some((BinaryOperator::And, 2, 1))
            }
            ExprToken::Word(word) if word == &self.vocabulary.or && self.level >= Level::Ten => {
                Some((BinaryOperator::Or, 1, 1))
            }
            ExprToken::Word(word)
                if word == &self.vocabulary.in_word
                    && (self.level <= Level::Twelve || self.level >= Level::Fourteen)
                    && self.level >= Level::Five =>
            {
                Some((BinaryOperator::In, 3, 1))
            }
            ExprToken::Word(word)
                if word == &self.vocabulary.not
                    && self.level >= Level::Six
                    && (self.level <= Level::Twelve || self.level >= Level::Fourteen)
                    && matches!(self.tokens.get(self.cursor + 1), Some(ExprToken::Word(next)) if next == &self.vocabulary.in_word) =>
            {
                Some((BinaryOperator::NotIn, 3, 2))
            }
            _ => None,
        }
    }

    fn parse_unary(&mut self) -> Result<Expression, Diagnostic> {
        if let Some(ExprToken::Operator(operator)) = self.tokens.get(self.cursor) {
            if operator == "-" {
                self.cursor += 1;
                return Ok(Expression::Unary(
                    UnaryOperator::Negate,
                    Box::new(self.parse_unary()?),
                ));
            }
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expression, Diagnostic> {
        let mut expression = self.parse_primary()?;
        loop {
            if matches!(self.tokens.get(self.cursor), Some(ExprToken::Word(word)) if word == &self.vocabulary.at)
                && self.level <= Level::Twelve
                && matches!(self.tokens.get(self.cursor + 1), Some(ExprToken::Word(word)) if word == &self.vocabulary.random)
            {
                self.cursor += 2;
                expression = Expression::RandomList(Box::new(expression));
            } else if self.tokens.get(self.cursor) == Some(&ExprToken::LeftBracket) {
                if self.level < Level::Fourteen {
                    return Err(Diagnostic {
                        span: self.span,
                        message: "bracket list access is introduced at Hedy level 14".to_owned(),
                    });
                }
                self.cursor += 1;
                let index = if matches!(self.tokens.get(self.cursor), Some(ExprToken::Word(word)) if word == &self.vocabulary.random)
                {
                    self.cursor += 1;
                    Expression::Name(self.vocabulary.random.clone())
                } else {
                    self.parse_binary(0)?
                };
                if self.tokens.get(self.cursor) != Some(&ExprToken::RightBracket) {
                    return Err(Diagnostic {
                        span: self.span,
                        message: "missing `]` after list index".to_owned(),
                    });
                }
                self.cursor += 1;
                if matches!(&index, Expression::Name(name) if name == &self.vocabulary.random) {
                    expression = Expression::RandomList(Box::new(expression));
                } else {
                    expression = Expression::Index(Box::new(expression), Box::new(index));
                }
            } else {
                break;
            }
        }
        Ok(expression)
    }

    fn parse_primary(&mut self) -> Result<Expression, Diagnostic> {
        let token = self
            .tokens
            .get(self.cursor)
            .cloned()
            .ok_or_else(|| Diagnostic {
                span: self.span,
                message: "expected an expression".to_owned(),
            })?;
        self.cursor += 1;
        match token {
            ExprToken::Number(number) => Ok(Expression::Literal(Value::Number(number))),
            ExprToken::Text(text) => Ok(Expression::Literal(Value::Text(text))),
            ExprToken::Word(word) if word.eq_ignore_ascii_case("true") => {
                Ok(Expression::Literal(Value::Boolean(true)))
            }
            ExprToken::Word(word) if word.eq_ignore_ascii_case("false") => {
                Ok(Expression::Literal(Value::Boolean(false)))
            }
            ExprToken::Word(word) => {
                if self.tokens.get(self.cursor) == Some(&ExprToken::LeftParen) {
                    self.cursor += 1;
                    let arguments = self.parse_arguments()?;
                    if word == self.vocabulary.input {
                        if self.level < Level::Thirteen {
                            return Err(Diagnostic {
                                span: self.span,
                                message: "input() is introduced at Hedy level 13".to_owned(),
                            });
                        }
                        return Ok(Expression::Input(arguments));
                    }
                    if word == self.vocabulary.range {
                        return Err(Diagnostic {
                            span: self.span,
                            message: "range is valid only in a for loop".to_owned(),
                        });
                    }
                    if self.level < Level::Fifteen {
                        return Err(Diagnostic {
                            span: self.span,
                            message:
                                "function calls with parentheses are introduced at Hedy level 15"
                                    .to_owned(),
                        });
                    }
                    return Ok(Expression::Call(word, arguments));
                }
                Ok(Expression::Name(word))
            }
            ExprToken::LeftParen => {
                let expression = self.parse_binary(0)?;
                if self.tokens.get(self.cursor) != Some(&ExprToken::RightParen) {
                    return Err(Diagnostic {
                        span: self.span,
                        message: "missing `)` in expression".to_owned(),
                    });
                }
                self.cursor += 1;
                Ok(expression)
            }
            ExprToken::LeftBracket => {
                if self.level < Level::Fourteen {
                    return Err(Diagnostic {
                        span: self.span,
                        message: "bracket lists are introduced at Hedy level 14".to_owned(),
                    });
                }
                let items = self.parse_arguments_until(ExprToken::RightBracket)?;
                if items.iter().any(|item| {
                    !matches!(item, Expression::Literal(Value::Text(_) | Value::Number(_)))
                        && !matches!(
                            item,
                            Expression::Unary(
                                UnaryOperator::Negate,
                                operand
                            ) if matches!(operand.as_ref(), Expression::Literal(Value::Number(_)))
                        )
                }) {
                    return Err(Diagnostic {
                        span: self.span,
                        message: "list literals contain quoted text or numeric values".to_owned(),
                    });
                }
                Ok(Expression::List(items))
            }
            _ => Err(Diagnostic {
                span: self.span,
                message: "expected a number, word, string, or parenthesized expression".to_owned(),
            }),
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>, Diagnostic> {
        self.parse_arguments_until(ExprToken::RightParen)
    }

    fn parse_arguments_until(
        &mut self,
        terminator: ExprToken,
    ) -> Result<Vec<Expression>, Diagnostic> {
        let mut arguments = Vec::new();
        if self.tokens.get(self.cursor) == Some(&terminator) {
            self.cursor += 1;
            return Ok(arguments);
        }
        loop {
            arguments.push(self.parse_binary(0)?);
            match self.tokens.get(self.cursor) {
                Some(ExprToken::Comma) => self.cursor += 1,
                Some(token) if *token == terminator => {
                    self.cursor += 1;
                    break;
                }
                _ => {
                    return Err(Diagnostic {
                        span: self.span,
                        message: "expected `,` or closing delimiter".to_owned(),
                    });
                }
            }
        }
        Ok(arguments)
    }
}

fn parse_expression(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Result<Expression, Diagnostic> {
    let tokens = lex_expression(source, span)?;
    ExpressionParser {
        tokens,
        cursor: 0,
        level,
        vocabulary,
        span,
    }
    .parse()
}

fn parse_condition(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Result<Expression, Diagnostic> {
    if source.trim().is_empty() {
        return Err(Diagnostic {
            span,
            message: "condition is empty".to_owned(),
        });
    }
    parse_expression(
        source.trim().trim_end_matches(':').trim(),
        level,
        vocabulary,
        span,
    )
}

fn parse_legacy_value(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Result<Expression, Diagnostic> {
    let source = source.trim();
    if level >= Level::Three {
        let items = split_list_items(source);
        if items.len() > 1 {
            return Ok(Expression::Literal(Value::List(
                items
                    .into_iter()
                    .map(|item| {
                        parse_legacy_value(item, level, vocabulary, span).and_then(|expression| {
                            match expression {
                                Expression::Literal(value) => Ok(value),
                                Expression::Name(name) => Ok(Value::Text(name)),
                                _ => Err(Diagnostic {
                                    span,
                                    message: "list items must be values at this Hedy level"
                                        .to_owned(),
                                }),
                            }
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )));
        }
    }
    if source == "" {
        return Err(Diagnostic {
            span,
            message: "assignment requires a value".to_owned(),
        });
    }
    let has_operator = source
        .chars()
        .any(|character| matches!(character, '+' | '-' | '*' | '/'))
        || source.contains(" at ")
        || source.starts_with('(')
        || source.starts_with('"')
        || source.starts_with('\'');
    if source.parse::<f64>().is_ok() || has_operator {
        return parse_expression(source, level, vocabulary, span);
    }
    if source.starts_with('"') || source.starts_with('\'') {
        return parse_expression(source, level, vocabulary, span);
    }
    if valid_identifier(source) {
        return Ok(Expression::Name(source.to_owned()));
    }
    Ok(Expression::Literal(Value::Text(source.to_owned())))
}

fn parse_statement(
    line: &str,
    span: Span,
    level: Level,
    vocabulary: &Vocabulary,
) -> Result<Statement, Diagnostic> {
    let invalid = |message: String| Diagnostic { span, message };

    if let Some(rest) = command_tail(line, &vocabulary.echo) {
        if level != Level::One {
            return Err(invalid("echo is available only at Hedy level 1".to_owned()));
        }
        return Ok(Statement {
            span,
            kind: StatementKind::Echo(rest.to_owned()),
        });
    }

    if let Some(rest) = command_tail(line, &vocabulary.print) {
        if rest.is_empty() {
            return Err(invalid("print requires text".to_owned()));
        }
        return Ok(Statement {
            span,
            kind: StatementKind::Print(parse_parts(rest, level, vocabulary, span)?),
        });
    }

    if let Some(rest) = command_tail(line, &vocabulary.ask) {
        if level != Level::One {
            return Err(invalid(format!(
                "at Hedy level {}, store input in a variable with `{}`",
                level.number(),
                vocabulary.is
            )));
        }
        if rest.is_empty() {
            return Err(invalid("ask requires a question".to_owned()));
        }
        return Ok(Statement {
            span,
            kind: StatementKind::Ask {
                target: None,
                prompt: parse_parts(rest, level, vocabulary, span)?,
            },
        });
    }

    if let Some(rest) = command_tail(line, &vocabulary.forward) {
        let kind = if rest.is_empty() {
            StatementKind::Forward(50.0)
        } else if level >= Level::Seven {
            StatementKind::ForwardExpression(parse_expression(rest, level, vocabulary, span)?)
        } else {
            StatementKind::Forward(parse_finite_number(rest, "forward", span)?)
        };
        return Ok(Statement { span, kind });
    }

    if let Some(rest) = command_tail(line, &vocabulary.turn) {
        let kind = if level == Level::One {
            match rest {
                value if value == vocabulary.left => StatementKind::Turn(Turn::Left),
                value if value == vocabulary.right || value.is_empty() => {
                    StatementKind::Turn(Turn::Right)
                }
                _ => {
                    return Err(invalid(
                        "level 1 turn accepts `left`, `right`, or no argument".to_owned(),
                    ));
                }
            }
        } else if rest.is_empty() {
            StatementKind::Turn(Turn::Right)
        } else if level >= Level::Seven {
            StatementKind::TurnExpression(parse_expression(rest, level, vocabulary, span)?)
        } else {
            StatementKind::Turn(Turn::Degrees(parse_finite_number(rest, "turn", span)?))
        };
        return Ok(Statement { span, kind });
    }

    if let Some(rest) = command_tail(line, &vocabulary.play) {
        if rest.is_empty() {
            return Err(invalid("play requires a note".to_owned()));
        }
        let kind = if valid_note(rest) {
            StatementKind::Play(rest.to_owned())
        } else if level >= Level::Seven {
            StatementKind::PlayExpression(parse_expression(rest, level, vocabulary, span)?)
        } else {
            return Err(invalid("play requires a musical note".to_owned()));
        };
        return Ok(Statement { span, kind });
    }

    if let Some(rest) = command_tail(line, &vocabulary.sleep) {
        if level < Level::Two {
            return Err(invalid("sleep is introduced at Hedy level 2".to_owned()));
        }
        let kind = if level >= Level::Seven {
            StatementKind::SleepExpression(parse_expression(rest, level, vocabulary, span)?)
        } else {
            let seconds = parse_finite_number(rest, "sleep", span)?;
            if seconds < 0.0 {
                return Err(invalid("sleep duration cannot be negative".to_owned()));
            }
            StatementKind::Sleep(seconds)
        };
        return Ok(Statement { span, kind });
    }

    if let Some(rest) = command_tail(line, &vocabulary.clear) {
        if level < Level::Four {
            return Err(invalid("clear is introduced at Hedy level 4".to_owned()));
        }
        if !rest.is_empty() {
            return Err(invalid("clear does not take arguments".to_owned()));
        }
        return Ok(Statement {
            span,
            kind: StatementKind::Clear,
        });
    }

    if let Some(rest) = command_tail(line, &vocabulary.color) {
        if level < Level::Four {
            return Err(invalid("color is introduced at Hedy level 4".to_owned()));
        }
        if rest.is_empty() {
            return Err(invalid("color requires a color name".to_owned()));
        }
        let kind = if is_turtle_color(rest) {
            StatementKind::Color(vec![Part::Literal(rest.to_owned())])
        } else if level >= Level::Seven {
            StatementKind::ColorExpression(parse_expression(rest, level, vocabulary, span)?)
        } else {
            StatementKind::Color(parse_parts(rest, level, vocabulary, span)?)
        };
        return Ok(Statement { span, kind });
    }

    if level >= Level::Two {
        if let Some((name, right)) = split_assignment(line, &vocabulary.is) {
            if !valid_identifier(name) {
                return Err(invalid(format!("invalid variable name `{name}`")));
            }
            if right.is_empty() {
                return Err(invalid("assignment requires a value".to_owned()));
            }
            if let Some(prompt) = command_tail(right, &vocabulary.ask) {
                if prompt.is_empty() {
                    return Err(invalid("ask requires a question".to_owned()));
                }
                return Ok(Statement {
                    span,
                    kind: StatementKind::Ask {
                        target: Some(name.to_owned()),
                        prompt: parse_parts(prompt, level, vocabulary, span)?,
                    },
                });
            }

            let value = if level >= Level::Three {
                let items = split_list_items(right);
                if items.len() > 1 {
                    Assignment::List(
                        items
                            .into_iter()
                            .map(|item| parse_parts(item, level, vocabulary, span))
                            .collect::<Result<Vec<_>, _>>()?,
                    )
                } else {
                    Assignment::Parts(parse_parts(right, level, vocabulary, span)?)
                }
            } else {
                Assignment::Parts(parse_parts(right, level, vocabulary, span)?)
            };

            return Ok(Statement {
                span,
                kind: StatementKind::Assign {
                    name: name.to_owned(),
                    value,
                },
            });
        }
    }

    if (Level::Three..Level::Thirteen).contains(&level) {
        if let Some((item, list)) = split_two_word_command(line, &vocabulary.add, &vocabulary.to) {
            if !valid_identifier(list) || item.is_empty() {
                return Err(invalid("add syntax is `add item to list`".to_owned()));
            }
            return Ok(Statement {
                span,
                kind: StatementKind::AddTo {
                    item: parse_parts(item, level, vocabulary, span)?,
                    list: list.to_owned(),
                },
            });
        }
        if let Some((item, list)) =
            split_two_word_command(line, &vocabulary.remove, &vocabulary.from)
        {
            if !valid_identifier(list) || item.is_empty() {
                return Err(invalid(
                    "remove syntax is `remove item from list`".to_owned(),
                ));
            }
            return Ok(Statement {
                span,
                kind: StatementKind::RemoveFrom {
                    item: parse_parts(item, level, vocabulary, span)?,
                    list: list.to_owned(),
                },
            });
        }
    }

    Err(invalid(format!(
        "unsupported syntax at Hedy level {}",
        level.number()
    )))
}

fn command_tail<'a>(line: &'a str, keyword: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(keyword)?;
    if rest.is_empty() {
        Some("")
    } else if rest.starts_with(char::is_whitespace) {
        Some(rest.trim())
    } else {
        None
    }
}

fn split_assignment<'a>(line: &'a str, keyword: &str) -> Option<(&'a str, &'a str)> {
    let separator = format!(" {keyword} ");
    let (name, right) = line.split_once(&separator)?;
    Some((name.trim(), right.trim()))
}

fn split_two_word_command<'a>(
    line: &'a str,
    first: &str,
    second: &str,
) -> Option<(&'a str, &'a str)> {
    let prefix = format!("{first} ");
    let rest = line.strip_prefix(&prefix)?;
    let separator = format!(" {second} ");
    let (item, list) = rest.rsplit_once(&separator)?;
    Some((item.trim(), list.trim()))
}

fn split_list_items(source: &str) -> Vec<&str> {
    let mut items = Vec::new();
    let mut start = 0;
    let mut quote = None;
    for (index, character) in source.char_indices() {
        match quote {
            Some(active) if is_quote_close(active, character) => quote = None,
            None if is_quote_open(character) => quote = Some(character),
            None if is_comma(character) => {
                items.push(source[start..index].trim());
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }
    items.push(source[start..].trim());
    items
}

fn parse_parts(
    source: &str,
    level: Level,
    vocabulary: &Vocabulary,
    span: Span,
) -> Result<Vec<Part>, Diagnostic> {
    if level < Level::Four {
        return Ok(vec![Part::Template(source.to_owned())]);
    }

    let mut parts = Vec::new();
    let mut cursor = 0;
    while cursor < source.len() {
        let character = source[cursor..]
            .chars()
            .next()
            .expect("cursor is in bounds");
        if character.is_whitespace() || is_comma(character) {
            cursor += character.len_utf8();
            continue;
        }

        if is_quote_open(character) {
            let quote = character;
            let start = cursor + quote.len_utf8();
            let Some((end, close_length)) = find_quote_close(source, start, quote) else {
                return Err(Diagnostic {
                    span,
                    message: "unterminated quoted text".to_owned(),
                });
            };
            parts.push(Part::Literal(unescape_string(&source[start..end])));
            cursor = end + close_length;
            continue;
        }

        let start = cursor;
        while cursor < source.len() {
            let character = source[cursor..]
                .chars()
                .next()
                .expect("cursor is in bounds");
            if character.is_whitespace() || is_comma(character) {
                break;
            }
            cursor += character.len_utf8();
        }
        let word = &source[start..cursor];
        if word == vocabulary.answer {
            parts.push(Part::Answer);
            continue;
        }
        if !valid_identifier(word) {
            return Err(Diagnostic {
                span,
                message: format!("text `{word}` must be quoted at Hedy level 4"),
            });
        }

        let after_word = skip_separators(source, cursor);
        if let Some((at, after_at)) = read_word(source, after_word) {
            let after_at = skip_separators(source, after_at);
            if at == vocabulary.at {
                if let Some((random, after_random)) = read_word(source, after_at) {
                    if random == vocabulary.random {
                        parts.push(Part::RandomList(word.to_owned()));
                        cursor = after_random;
                        continue;
                    }
                }
            }
        }
        parts.push(Part::Variable(word.to_owned()));
    }

    if parts.is_empty() {
        return Err(Diagnostic {
            span,
            message: "text expression is empty".to_owned(),
        });
    }
    Ok(parts)
}

fn skip_separators(source: &str, mut cursor: usize) -> usize {
    while cursor < source.len() {
        let character = source[cursor..]
            .chars()
            .next()
            .expect("cursor is in bounds");
        if character.is_whitespace() || is_comma(character) {
            cursor += character.len_utf8();
        } else {
            break;
        }
    }
    cursor
}

fn read_word(source: &str, cursor: usize) -> Option<(&str, usize)> {
    if cursor >= source.len() {
        return None;
    }
    let rest = &source[cursor..];
    let end = rest
        .char_indices()
        .find_map(|(index, character)| {
            if character.is_whitespace() || is_comma(character) {
                Some(index)
            } else {
                None
            }
        })
        .unwrap_or(rest.len());
    if end == 0 {
        None
    } else {
        Some((&rest[..end], cursor + end))
    }
}

fn parse_finite_number(source: &str, command: &str, span: Span) -> Result<f64, Diagnostic> {
    let value = source.parse::<f64>().map_err(|_| Diagnostic {
        span,
        message: format!("{command} requires a numeric literal"),
    })?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(Diagnostic {
            span,
            message: format!("{command} requires a finite number"),
        })
    }
}

fn valid_identifier(name: &str) -> bool {
    let mut characters = name.chars();
    matches!(characters.next(), Some(character) if character.is_alphabetic() || character == '_')
        && characters.all(|character| character.is_alphanumeric() || character == '_')
}

fn valid_note(note: &str) -> bool {
    let bytes = note.as_bytes();
    matches!(bytes.first(), Some(b'A'..=b'G'))
        && (bytes.len() == 1
            || (bytes.len() == 2 && bytes[1].is_ascii_digit())
            || (bytes.len() == 3 && matches!(bytes[1], b'#' | b'b') && bytes[2].is_ascii_digit()))
}

fn is_turtle_color(color: &str) -> bool {
    matches!(
        color,
        "black"
            | "blue"
            | "brown"
            | "gray"
            | "green"
            | "orange"
            | "pink"
            | "purple"
            | "red"
            | "white"
            | "yellow"
    )
}

pub struct Execution {
    pub variables: HashMap<String, Value>,
    pub last_answer: Option<String>,
    pub executed_statements: usize,
}

#[derive(Clone)]
struct Function {
    parameters: Vec<String>,
    body: Vec<Statement>,
}

#[derive(Default)]
struct Environment {
    variables: HashMap<String, Value>,
    scopes: Vec<HashMap<String, Value>>,
    functions: HashMap<String, Function>,
    last_answer: Option<String>,
    executed_statements: usize,
}

impl Environment {
    fn get(&self, name: &str) -> Option<&Value> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .or_else(|| self.variables.get(name))
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Value> {
        if let Some(scope) = self
            .scopes
            .iter_mut()
            .rev()
            .find(|scope| scope.contains_key(name))
        {
            scope.get_mut(name)
        } else {
            self.variables.get_mut(name)
        }
    }

    fn set(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        } else {
            self.variables.insert(name, value);
        }
    }
}

enum Flow {
    Continue,
    Return(Option<Value>),
}

pub struct Interpreter<H> {
    host: H,
    statement_limit: usize,
    call_depth: usize,
}

impl<H: Host> Interpreter<H> {
    pub fn new(host: H) -> Self {
        Self {
            host,
            statement_limit: 1_000_000,
            call_depth: 0,
        }
    }

    /// Set an execution limit for loops and recursive calls.
    pub fn with_statement_limit(mut self, limit: usize) -> Self {
        self.statement_limit = limit;
        self
    }

    pub fn run(&mut self, program: &Program) -> Result<Execution, RuntimeError> {
        let mut environment = Environment::default();
        match self.execute_block(&program.statements, program, &mut environment)? {
            Flow::Continue => {}
            Flow::Return(_) => {
                return Err(runtime_error(
                    Span::default(),
                    "return can only be used inside a function".to_owned(),
                ));
            }
        }
        Ok(Execution {
            variables: environment.variables,
            last_answer: environment.last_answer,
            executed_statements: environment.executed_statements,
        })
    }

    fn execute_block(
        &mut self,
        statements: &[Statement],
        program: &Program,
        environment: &mut Environment,
    ) -> Result<Flow, RuntimeError> {
        for statement in statements {
            if let StatementKind::Define {
                name,
                parameters,
                body,
            } = &statement.kind
            {
                if environment.functions.contains_key(name) {
                    return Err(runtime_error(
                        statement.span,
                        format!("function `{name}` is defined more than once"),
                    ));
                }
                environment.functions.insert(
                    name.clone(),
                    Function {
                        parameters: parameters.clone(),
                        body: body.clone(),
                    },
                );
                continue;
            }
            if environment.executed_statements >= self.statement_limit {
                return Err(runtime_error(
                    statement.span,
                    format!(
                        "execution stopped after {} statements",
                        self.statement_limit
                    ),
                ));
            }
            environment.executed_statements += 1;
            match self.execute_statement(statement, program, environment)? {
                Flow::Continue => {}
                returned @ Flow::Return(_) => return Ok(returned),
            }
        }
        Ok(Flow::Continue)
    }

    fn execute_statement(
        &mut self,
        statement: &Statement,
        program: &Program,
        environment: &mut Environment,
    ) -> Result<Flow, RuntimeError> {
        let span = statement.span;
        match &statement.kind {
            StatementKind::Print(parts) => {
                let text = self.render_parts(parts, program, environment, span)?;
                self.host
                    .output(&text)
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::PrintExpressions(expressions) => {
                let text = expressions
                    .iter()
                    .map(|expression| self.evaluate(expression, program, environment, span))
                    .collect::<Result<Vec<_>, _>>()?
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                self.host
                    .output(&text)
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::Ask { target, prompt } => {
                let text = self.render_parts(prompt, program, environment, span)?;
                let answer = self
                    .host
                    .input(&text)
                    .map_err(|message| runtime_error(span, message))?;
                environment.last_answer = Some(answer.clone());
                environment.set(
                    program.vocabulary.answer.clone(),
                    Value::Text(answer.clone()),
                );
                if let Some(name) = target {
                    environment.set(name.clone(), Value::Text(answer));
                }
            }
            StatementKind::Assign { name, value } => {
                let value = match value {
                    Assignment::Parts(parts) => {
                        Value::Text(self.render_parts(parts, program, environment, span)?)
                    }
                    Assignment::List(items) => Value::List(
                        items
                            .iter()
                            .map(|parts| {
                                self.render_parts(parts, program, environment, span)
                                    .map(Value::Text)
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                };
                environment.set(name.clone(), value);
            }
            StatementKind::AddTo { item, list } => {
                let item = Value::Text(self.render_parts(item, program, environment, span)?);
                let Some(Value::List(items)) = environment.get_mut(list) else {
                    return Err(runtime_error(
                        span,
                        format!("`{list}` is not a declared list"),
                    ));
                };
                items.push(item);
            }
            StatementKind::RemoveFrom { item, list } => {
                let item = Value::Text(self.render_parts(item, program, environment, span)?);
                let Some(Value::List(items)) = environment.get_mut(list) else {
                    return Err(runtime_error(
                        span,
                        format!("`{list}` is not a declared list"),
                    ));
                };
                if let Some(index) = items.iter().position(|candidate| candidate == &item) {
                    items.remove(index);
                } else {
                    return Err(runtime_error(
                        span,
                        format!("`{item}` is not in list `{list}`"),
                    ));
                }
            }
            StatementKind::Sleep(seconds) => self
                .host
                .sleep(*seconds)
                .map_err(|message| runtime_error(span, message))?,
            StatementKind::SleepExpression(expression) => {
                let seconds = numeric_value(
                    self.evaluate(expression, program, environment, span)?,
                    "sleep",
                    span,
                )?;
                if seconds < 0.0 {
                    return Err(runtime_error(
                        span,
                        "sleep duration cannot be negative".to_owned(),
                    ));
                }
                self.host
                    .sleep(seconds)
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::Forward(steps) => self
                .host
                .turtle_forward(*steps)
                .map_err(|message| runtime_error(span, message))?,
            StatementKind::ForwardExpression(expression) => {
                let steps = numeric_value(
                    self.evaluate(expression, program, environment, span)?,
                    "forward",
                    span,
                )?;
                self.host
                    .turtle_forward(steps)
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::Turn(turn) => self
                .host
                .turtle_turn(*turn)
                .map_err(|message| runtime_error(span, message))?,
            StatementKind::TurnExpression(expression) => {
                let degrees = numeric_value(
                    self.evaluate(expression, program, environment, span)?,
                    "turn",
                    span,
                )?;
                self.host
                    .turtle_turn(Turn::Degrees(degrees))
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::Play(note) => self
                .host
                .play_note(note)
                .map_err(|message| runtime_error(span, message))?,
            StatementKind::PlayExpression(expression) => {
                let note = self
                    .evaluate(expression, program, environment, span)?
                    .to_string();
                self.host
                    .play_note(&note)
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::Clear => self
                .host
                .turtle_clear()
                .map_err(|message| runtime_error(span, message))?,
            StatementKind::Color(parts) => {
                let color = self.render_parts(parts, program, environment, span)?;
                self.host
                    .turtle_color(&color)
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::ColorExpression(expression) => {
                let color = self
                    .evaluate(expression, program, environment, span)?
                    .to_string();
                self.host
                    .turtle_color(&color)
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::Echo(prefix) => {
                let answer = environment.last_answer.as_deref().ok_or_else(|| {
                    runtime_error(span, "echo is used before any ask command".to_owned())
                })?;
                let text = format!("{prefix} {answer}");
                self.host
                    .output(&text)
                    .map_err(|message| runtime_error(span, message))?;
            }
            StatementKind::AssignExpression { name, expression } => {
                let value = self.evaluate(expression, program, environment, span)?;
                environment.set(name.clone(), value);
            }
            StatementKind::If {
                branches,
                otherwise,
            } => {
                for (condition, body) in branches {
                    if self
                        .evaluate(condition, program, environment, span)?
                        .as_boolean()
                        .map_err(|message| runtime_error(span, message))?
                    {
                        return self.execute_block(body, program, environment);
                    }
                }
                return self.execute_block(otherwise, program, environment);
            }
            StatementKind::IfPressed {
                key,
                body,
                otherwise,
            } => {
                let pressed = self
                    .host
                    .key_pressed(key)
                    .map_err(|message| runtime_error(span, message))?;
                if pressed {
                    return self.execute_block(body, program, environment);
                }
                return self.execute_block(otherwise, program, environment);
            }
            StatementKind::Repeat { count, body } => {
                let count = integer_value(
                    self.evaluate(count, program, environment, span)?,
                    "repeat count",
                    span,
                )?;
                if count < 0 {
                    return Err(runtime_error(
                        span,
                        "repeat count cannot be negative".to_owned(),
                    ));
                }
                for _ in 0..count {
                    match self.execute_block(body, program, environment)? {
                        Flow::Continue => {}
                        returned @ Flow::Return(_) => return Ok(returned),
                    }
                }
            }
            StatementKind::ForList {
                variable,
                list,
                body,
            } => {
                let Some(Value::List(items)) = environment.get(list).cloned() else {
                    return Err(runtime_error(span, format!("`{list}` is not a list")));
                };
                for item in items {
                    environment.set(variable.clone(), item);
                    match self.execute_block(body, program, environment)? {
                        Flow::Continue => {}
                        returned @ Flow::Return(_) => return Ok(returned),
                    }
                }
            }
            StatementKind::ForRange {
                variable,
                start,
                end,
                step,
                end_inclusive,
                body,
            } => {
                let start = integer_value(
                    self.evaluate(start, program, environment, span)?,
                    "range start",
                    span,
                )?;
                let end = integer_value(
                    self.evaluate(end, program, environment, span)?,
                    "range end",
                    span,
                )?;
                let step = if let Some(step) = step {
                    integer_value(
                        self.evaluate(step, program, environment, span)?,
                        "range step",
                        span,
                    )?
                } else {
                    1
                };
                if step == 0 {
                    return Err(runtime_error(span, "range step cannot be zero".to_owned()));
                }
                let mut current = start;
                while if step > 0 {
                    if *end_inclusive {
                        current <= end
                    } else {
                        current < end
                    }
                } else if *end_inclusive {
                    current >= end
                } else {
                    current > end
                } {
                    environment.set(variable.clone(), Value::Number(current as f64));
                    match self.execute_block(body, program, environment)? {
                        Flow::Continue => {}
                        returned @ Flow::Return(_) => return Ok(returned),
                    }
                    current = current
                        .checked_add(step)
                        .ok_or_else(|| runtime_error(span, "range value overflow".to_owned()))?;
                }
            }
            StatementKind::While { condition, body } => loop {
                if !self
                    .evaluate(condition, program, environment, span)?
                    .as_boolean()
                    .map_err(|message| runtime_error(span, message))?
                {
                    break;
                }
                match self.execute_block(body, program, environment)? {
                    Flow::Continue => {}
                    returned @ Flow::Return(_) => return Ok(returned),
                }
            },
            StatementKind::Define { .. } => {}
            StatementKind::Call { name, arguments } => {
                let _ = self.call_function(name, arguments, program, environment, span)?;
            }
            StatementKind::Return(expression) => {
                if self.call_depth == 0 {
                    return Err(runtime_error(
                        span,
                        "return can only be used inside a function".to_owned(),
                    ));
                }
                let value = expression
                    .as_ref()
                    .map(|expression| self.evaluate(expression, program, environment, span))
                    .transpose()?;
                return Ok(Flow::Return(value));
            }
        }
        Ok(Flow::Continue)
    }

    fn render_parts(
        &mut self,
        parts: &[Part],
        program: &Program,
        environment: &mut Environment,
        span: Span,
    ) -> Result<String, RuntimeError> {
        let mut result = String::new();
        for part in parts {
            match part {
                Part::Template(template) => {
                    result.push_str(&self.render_template(template, program, environment, span)?)
                }
                Part::Literal(text) => result.push_str(text),
                Part::Answer => {
                    result.push_str(environment.last_answer.as_deref().ok_or_else(|| {
                        runtime_error(span, "answer is used before any ask command".to_owned())
                    })?)
                }
                Part::Variable(name) => match environment.get(name) {
                    Some(Value::List(values)) if program.level < Level::Fourteen => {
                        return Err(runtime_error(
                            span,
                            format!("list `{name}` cannot be printed as text"),
                        ));
                    }
                    Some(value) => result.push_str(&value.to_string()),
                    None => {
                        return Err(runtime_error(span, format!("undefined variable `{name}`")));
                    }
                },
                Part::RandomList(name) => {
                    let Some(Value::List(items)) = environment.get(name).cloned() else {
                        return Err(runtime_error(
                            span,
                            format!("`{name}` is not a declared list"),
                        ));
                    };
                    if items.is_empty() {
                        return Err(runtime_error(
                            span,
                            format!("cannot select a random item from empty list `{name}`"),
                        ));
                    }
                    let index = self
                        .host
                        .random_index(items.len())
                        .map_err(|message| runtime_error(span, message))?;
                    let item = items.get(index).ok_or_else(|| {
                        runtime_error(
                            span,
                            "host returned an invalid random list index".to_owned(),
                        )
                    })?;
                    result.push_str(&item.to_string());
                }
            }
        }
        Ok(result)
    }

    fn render_template(
        &mut self,
        template: &str,
        program: &Program,
        environment: &mut Environment,
        span: Span,
    ) -> Result<String, RuntimeError> {
        let mut result = String::new();
        let mut cursor = 0;
        while cursor < template.len() {
            if let Some((word, word_end)) = read_identifier(template, cursor) {
                let at_start = skip_separators(template, word_end);
                if let Some((at, at_end)) = read_word(template, at_start) {
                    let random_start = skip_separators(template, at_end);
                    if at == program.vocabulary.at {
                        if let Some((random, random_end)) = read_word(template, random_start) {
                            if random == program.vocabulary.random {
                                let Some(Value::List(items)) = environment.get(word).cloned()
                                else {
                                    return Err(runtime_error(
                                        span,
                                        format!("`{word}` is not a declared list"),
                                    ));
                                };
                                if items.is_empty() {
                                    return Err(runtime_error(
                                        span,
                                        format!(
                                            "cannot select a random item from empty list `{word}`"
                                        ),
                                    ));
                                }
                                let index = self
                                    .host
                                    .random_index(items.len())
                                    .map_err(|message| runtime_error(span, message))?;
                                let item = items.get(index).ok_or_else(|| {
                                    runtime_error(
                                        span,
                                        "host returned an invalid random list index".to_owned(),
                                    )
                                })?;
                                result.push_str(&item.to_string());
                                cursor = random_end;
                                continue;
                            }
                        }
                    }
                }
                if word == program.vocabulary.answer {
                    if let Some(answer) = &environment.last_answer {
                        result.push_str(answer);
                    } else if let Some(Value::Text(answer)) = environment.get(word) {
                        result.push_str(answer);
                    } else {
                        return Err(runtime_error(
                            span,
                            "answer is used before any ask command".to_owned(),
                        ));
                    }
                } else if program.level >= Level::Two {
                    match environment.get(word) {
                        Some(Value::List(_)) if program.level < Level::Fourteen => {
                            return Err(runtime_error(
                                span,
                                format!("list `{word}` cannot be printed as text"),
                            ));
                        }
                        Some(value) => result.push_str(&value.to_string()),
                        None => result.push_str(word),
                    }
                } else {
                    result.push_str(word);
                }
                cursor = word_end;
            } else {
                let character = template[cursor..]
                    .chars()
                    .next()
                    .expect("cursor is in bounds");
                result.push(character);
                cursor += character.len_utf8();
            }
        }
        Ok(result)
    }

    fn evaluate(
        &mut self,
        expression: &Expression,
        program: &Program,
        environment: &mut Environment,
        span: Span,
    ) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Literal(value) => Ok(value.clone()),
            Expression::List(expressions) => expressions
                .iter()
                .map(|expression| self.evaluate(expression, program, environment, span))
                .collect::<Result<Vec<_>, _>>()
                .map(Value::List),
            Expression::Concatenate(expressions) => {
                let mut text = String::new();
                for expression in expressions {
                    text.push_str(
                        &self
                            .evaluate(expression, program, environment, span)?
                            .to_string(),
                    );
                }
                Ok(Value::Text(text))
            }
            Expression::Name(name) => match environment.get(name) {
                Some(value) => Ok(value.clone()),
                None if program.level < Level::Twelve => Ok(Value::Text(name.clone())),
                None => Err(runtime_error(span, format!("undefined variable `{name}`"))),
            },
            Expression::RandomList(list_expression) => {
                let value = self.evaluate(list_expression, program, environment, span)?;
                let Value::List(items) = value else {
                    return Err(runtime_error(
                        span,
                        "random selection requires a list".to_owned(),
                    ));
                };
                if items.is_empty() {
                    return Err(runtime_error(
                        span,
                        "cannot choose from an empty list".to_owned(),
                    ));
                }
                let index = self
                    .host
                    .random_index(items.len())
                    .map_err(|message| runtime_error(span, message))?;
                items.get(index).cloned().ok_or_else(|| {
                    runtime_error(
                        span,
                        "host returned an invalid random list index".to_owned(),
                    )
                })
            }
            Expression::Index(list_expression, index_expression) => {
                let value = self.evaluate(list_expression, program, environment, span)?;
                let index = integer_value(
                    self.evaluate(index_expression, program, environment, span)?,
                    "list index",
                    span,
                )?;
                let Value::List(items) = value else {
                    return Err(runtime_error(span, "indexing requires a list".to_owned()));
                };
                let resolved = if index < 0 {
                    items.len() as i64 + index
                } else {
                    index
                };
                usize::try_from(resolved)
                    .ok()
                    .and_then(|index| items.get(index))
                    .cloned()
                    .ok_or_else(|| {
                        runtime_error(span, format!("list index {index} is out of range"))
                    })
            }
            Expression::Unary(operator, operand) => {
                let value = self.evaluate(operand, program, environment, span)?;
                match operator {
                    UnaryOperator::Negate => {
                        Ok(Value::Number(-numeric_value(value, "unary minus", span)?))
                    }
                }
            }
            Expression::Binary(left, BinaryOperator::And, right) => {
                let left = boolean_value(self.evaluate(left, program, environment, span)?, span)?;
                if !left {
                    Ok(Value::Boolean(false))
                } else {
                    Ok(Value::Boolean(boolean_value(
                        self.evaluate(right, program, environment, span)?,
                        span,
                    )?))
                }
            }
            Expression::Binary(left, BinaryOperator::Or, right) => {
                let left = boolean_value(self.evaluate(left, program, environment, span)?, span)?;
                if left {
                    Ok(Value::Boolean(true))
                } else {
                    Ok(Value::Boolean(boolean_value(
                        self.evaluate(right, program, environment, span)?,
                        span,
                    )?))
                }
            }
            Expression::Binary(left, operator, right) => {
                let left = self.evaluate(left, program, environment, span)?;
                let right = self.evaluate(right, program, environment, span)?;
                evaluate_binary(left, *operator, right, span)
            }
            Expression::Input(arguments) => {
                let mut prompt = String::new();
                for argument in arguments {
                    prompt.push_str(
                        &self
                            .evaluate(argument, program, environment, span)?
                            .to_string(),
                    );
                }
                let answer = self
                    .host
                    .input(&prompt)
                    .map_err(|message| runtime_error(span, message))?;
                Ok(Value::Text(answer))
            }
            Expression::Call(name, arguments) => {
                self.call_function(name, arguments, program, environment, span)
            }
        }
    }

    fn call_function(
        &mut self,
        name: &str,
        arguments: &[Expression],
        program: &Program,
        environment: &mut Environment,
        span: Span,
    ) -> Result<Value, RuntimeError> {
        if self.call_depth >= 256 {
            return Err(runtime_error(
                span,
                "function call depth limit exceeded".to_owned(),
            ));
        }
        let function = environment
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| runtime_error(span, format!("undefined function `{name}`")))?;
        if function.parameters.len() != arguments.len() {
            return Err(runtime_error(
                span,
                format!(
                    "function `{name}` expects {} argument(s), received {}",
                    function.parameters.len(),
                    arguments.len()
                ),
            ));
        }
        let mut scope = HashMap::new();
        for (parameter, argument) in function.parameters.iter().zip(arguments) {
            scope.insert(
                parameter.clone(),
                self.evaluate(argument, program, environment, span)?,
            );
        }
        environment.scopes.push(scope);
        self.call_depth += 1;
        let result = self.execute_block(&function.body, program, environment);
        self.call_depth -= 1;
        environment.scopes.pop();
        match result? {
            Flow::Continue => Ok(Value::Text(String::new())),
            Flow::Return(Some(value)) => Ok(value),
            Flow::Return(None) => Ok(Value::Text(String::new())),
        }
    }
}

impl Value {
    fn as_boolean(&self) -> Result<bool, String> {
        match self {
            Self::Boolean(value) => Ok(*value),
            _ => Err(format!("expected a boolean, found `{self}`")),
        }
    }
}

fn runtime_error(span: Span, message: String) -> RuntimeError {
    RuntimeError { span, message }
}

fn boolean_value(value: Value, span: Span) -> Result<bool, RuntimeError> {
    value
        .as_boolean()
        .map_err(|message| runtime_error(span, message))
}

fn numeric_value(value: Value, context: &str, span: Span) -> Result<f64, RuntimeError> {
    let number = match value {
        Value::Number(number) => number,
        Value::Text(text) => text.parse::<f64>().map_err(|_| {
            runtime_error(span, format!("{context} requires a number, found `{text}`"))
        })?,
        other => {
            return Err(runtime_error(
                span,
                format!("{context} requires a number, found `{other}`"),
            ));
        }
    };
    if number.is_finite() {
        Ok(number)
    } else {
        Err(runtime_error(
            span,
            format!("{context} produced a non-finite number"),
        ))
    }
}

fn integer_value(value: Value, context: &str, span: Span) -> Result<i64, RuntimeError> {
    let number = numeric_value(value, context, span)?;
    if number.fract() != 0.0 || number < i64::MIN as f64 || number > i64::MAX as f64 {
        return Err(runtime_error(span, format!("{context} must be an integer")));
    }
    Ok(number as i64)
}

fn evaluate_binary(
    left: Value,
    operator: BinaryOperator,
    right: Value,
    span: Span,
) -> Result<Value, RuntimeError> {
    let value = match operator {
        BinaryOperator::Add => match (&left, &right) {
            (Value::Text(left), Value::Text(right)) => {
                match (left.parse::<f64>(), right.parse::<f64>()) {
                    (Ok(left), Ok(right)) => Value::Number(left + right),
                    _ => Value::Text(format!("{left}{right}")),
                }
            }
            (Value::Number(_), _) | (_, Value::Number(_)) => Value::Number(
                numeric_value(left, "addition", span)? + numeric_value(right, "addition", span)?,
            ),
            _ => Value::Text(format!("{left}{right}")),
        },
        BinaryOperator::Subtract => Value::Number(
            numeric_value(left, "subtraction", span)? - numeric_value(right, "subtraction", span)?,
        ),
        BinaryOperator::Multiply => Value::Number(
            numeric_value(left, "multiplication", span)?
                * numeric_value(right, "multiplication", span)?,
        ),
        BinaryOperator::Divide => {
            let divisor = numeric_value(right, "division", span)?;
            if divisor == 0.0 {
                return Err(runtime_error(span, "division by zero".to_owned()));
            }
            Value::Number(numeric_value(left, "division", span)? / divisor)
        }
        BinaryOperator::Equal | BinaryOperator::NotEqual => {
            let equal = values_equal(&left, &right);
            Value::Boolean(if matches!(operator, BinaryOperator::Equal) {
                equal
            } else {
                !equal
            })
        }
        BinaryOperator::Less
        | BinaryOperator::LessEqual
        | BinaryOperator::Greater
        | BinaryOperator::GreaterEqual => {
            let ordering = compare_values(&left, &right, span)?;
            Value::Boolean(match operator {
                BinaryOperator::Less => ordering.is_lt(),
                BinaryOperator::LessEqual => !ordering.is_gt(),
                BinaryOperator::Greater => ordering.is_gt(),
                BinaryOperator::GreaterEqual => !ordering.is_lt(),
                _ => unreachable!(),
            })
        }
        BinaryOperator::In | BinaryOperator::NotIn => {
            let Value::List(items) = right else {
                return Err(runtime_error(
                    span,
                    "`in` requires a list on the right".to_owned(),
                ));
            };
            let contains = items.iter().any(|item| values_equal(&left, item));
            Value::Boolean(if matches!(operator, BinaryOperator::In) {
                contains
            } else {
                !contains
            })
        }
        BinaryOperator::And | BinaryOperator::Or => {
            unreachable!("short-circuit operators are handled before this function")
        }
    };
    if matches!(value, Value::Number(number) if !number.is_finite()) {
        return Err(runtime_error(
            span,
            "arithmetic produced a non-finite number".to_owned(),
        ));
    }
    Ok(value)
}

fn values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Text(right)) | (Value::Text(right), Value::Number(left)) => {
            right.parse::<f64>().is_ok_and(|right| *left == right)
        }
        _ => left == right,
    }
}

fn compare_values(
    left: &Value,
    right: &Value,
    span: Span,
) -> Result<std::cmp::Ordering, RuntimeError> {
    match (left, right) {
        (Value::Text(left), Value::Text(right)) => {
            match (left.parse::<f64>(), right.parse::<f64>()) {
                (Ok(left), Ok(right)) => left
                    .partial_cmp(&right)
                    .ok_or_else(|| runtime_error(span, "values cannot be compared".to_owned())),
                _ => Ok(left.cmp(right)),
            }
        }
        (Value::Boolean(left), Value::Boolean(right)) => Ok(left.cmp(right)),
        _ => {
            let left = numeric_value(left.clone(), "comparison", span)?;
            let right = numeric_value(right.clone(), "comparison", span)?;
            left.partial_cmp(&right)
                .ok_or_else(|| runtime_error(span, "values cannot be compared".to_owned()))
        }
    }
}

fn read_identifier(source: &str, cursor: usize) -> Option<(&str, usize)> {
    if cursor >= source.len() {
        return None;
    }
    let first = source[cursor..].chars().next()?;
    if !(first.is_alphabetic() || first == '_') {
        return None;
    }
    let mut end = cursor + first.len_utf8();
    while end < source.len() {
        let character = source[end..].chars().next()?;
        if !character.is_alphanumeric() && character != '_' {
            break;
        }
        end += character.len_utf8();
    }
    Some((&source[cursor..end], end))
}

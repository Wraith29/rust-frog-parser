import std/tables
import std/strutils
import std/sugar

type
  LineParserObj = object
    line: string
  
  LineParser = ref LineParserObj

  ParserObj = object
    templ: string

  Parser = ref ParserObj

proc newLineParser(line: string): LineParser =
  result = new LineParserObj
  result.line = line

proc newParser(templ: string): Parser =
  result = new ParserObj
  result.templ = templ

proc parse(self: LineParser, variables: Table[string, auto]): string = 
  if self.line.contains("{|") and self.line.contains("|}"):
    var 
      newStr = ""
      startIdx = self.line.find("{|")
      endIdx = self.line.find("|}")
    
    if startIdx > 0:
      newStr.add(self.line[0 .. startIdx - 1])
    
    var key = self.line[startIdx + 2 .. endIdx - 1]

    if variables.hasKey(key):
      newStr.add(variables[key])
    else:
      newStr.add(key)

    if endIdx < self.line.len():
      newStr.add(self.line[endIdx + 2 .. self.line.len() - 1])
    
    return newStr.strip()
  self.line.strip()

proc parse(self: Parser, vars: Table[string, auto]): string =
  var lines = collect:
    self.templ.split('\n')
  
  lines = collect:
    for line in lines:
      let lineParser = newLineParser(line)
      lineParser.parse(vars)

  lines.join("\n")

proc parseTemplate*(templ: string, vars: Table[string, auto]): string =
  let parser = newParser(templ)
  parser.parse(vars)
#include <iostream>
#include <string>
#include <map>
#include <vector>
#include <stack>

// enum Symbol : unsigned int {
// 	// the symbols:
// 	// Terminal symbols:
// 	TS_L_PARENS, // (
// 	TS_R_PARENS, // )
// 	TS_A, // a
// 	TS_NBR, // any positive number
// 	TS_PLUS, // +
// 	TS_LESS, // -
// 	TS_TIMES, // *
// 	TS_EOS, // $, in this case corresponds to '\0'
// 	TS_INVALID, // invalid token

//  	// Non-terminal symbols:
//  	NTS_EXPR, // S
//  	NTS_VALUE, // F
//  	NTS_SIGN // F
// };

// static const std::string SYMBOLS_NAME[] = {
// 	[TS_L_PARENS] = "TS_L_PARENS",
// 	[TS_R_PARENS] = "TS_R_PARENS",
// 	[TS_A] = "TS_A",
// 	[TS_NBR] = "TS_NBR",
// 	[TS_PLUS] = "TS_PLUS",
// 	[TS_LESS] = "TS_LESS",
// 	[TS_TIMES] = "TS_TIMES",
// 	[TS_EOS] = "TS_EOS",
// 	[TS_INVALID] = "TS_INVALID",
// 	[NTS_EXPR] = "NTS_EXPR",
// 	[NTS_VALUE] = "NTS_VALUE",
// 	[NTS_SIGN] = "NTS_SIGN"
// };

struct Symbol {
	const std::string name;

	Symbol(std::string name)
	: name(name) {};
};

// static const Symbol SYMBOLS[] = {
// 	Symbol("TS_A")
// };

static const Symbol TS_L_PARENS("TS_L_PARENS");
static const Symbol TS_R_PARENS("TS_R_PARENS");
static const Symbol TS_A("TS_A");
static const Symbol TS_NBR("TS_NBR");
static const Symbol TS_PLUS("TS_PLUS");
static const Symbol TS_LESS("TS_LESS");
static const Symbol TS_TIMES("TS_TIMES");
static const Symbol TS_EOS("TS_EOS");
static const Symbol TS_INVALID("TS_INVALID");
static const Symbol NTS_EXPR("NTS_EXPR");
static const Symbol NTS_VALUE("NTS_VALUE");
static const Symbol NTS_SIGN("NTS_SIGN");

using SymbolStack = std::stack<Symbol>;

struct Rule {
	const Symbol *sym1;
	const Symbol *sym2;
	std::vector<const Symbol *> res;

	Rule(const Symbol *sym1, const Symbol *sym2, std::initializer_list<const Symbol *> syms)
	: sym1(sym1), sym2(sym2) {
		for (auto &i : syms) {
			res.push_back(i);
		}
	};
};

class SymbolRuleTable {
public:
	SymbolRuleTable(const std::vector<Rule> &&rules) {
		for (auto &rule : rules) {
			for (auto &sym : rule.res) {
				_rules[rule.sym1][rule.sym2].push_back(sym);
			}
		}
	}

	const std::vector<const Symbol *> &getRes(const Symbol *sym1, const Symbol *sym2) const {
		return _rules.at(sym1).at(sym2);
	}

private:
	std::map<const Symbol *, std::map<const Symbol *, std::vector<const Symbol *>>> _rules;
};

SymbolRuleTable srt({
	Rule(&NTS_EXPR, &TS_A, {&NTS_VALUE}),
	Rule(&NTS_EXPR, &TS_NBR, {&NTS_VALUE}),
	Rule(&NTS_EXPR, &TS_L_PARENS, {
		&TS_R_PARENS,
		&NTS_EXPR,
		&NTS_SIGN,
		&NTS_EXPR,
		&TS_L_PARENS
	}),
	Rule(&NTS_VALUE, &TS_A, {&TS_A}),
	Rule(&NTS_VALUE, &TS_NBR, {&TS_NBR}),
	Rule(&NTS_SIGN, &TS_PLUS, {&TS_PLUS}),
	Rule(&NTS_SIGN, &TS_LESS, {&TS_LESS}),
	Rule(&NTS_SIGN, &TS_TIMES, {&TS_TIMES}),
});

class Lexer {
private:
	const Symbol *getSymbol(char c) {
		if (isdigit(c))
			return &TS_NBR;

		switch (c) {
		case '(':
			return &TS_L_PARENS;
		case ')':
			return &TS_R_PARENS;
		case 'a':
			return &TS_A;
		case '+':
			return &TS_PLUS;
		case '-':
			return &TS_LESS;
		case '*':
			return &TS_TIMES;
		case '\0':
			return &TS_EOS;
		default:
			return &TS_INVALID;
		}
	}

public:
	Lexer(const SymbolRuleTable &rules)
	: _rules(rules) {
		_ss.push(&TS_EOS);
		_ss.push(&NTS_EXPR);
	}

	std::vector<const Symbol *> parse(const char *str) {
		std::vector<const Symbol *> _syms;

		while (_ss.size()) {
			const Symbol *sym = getSymbol(*str);

			if (sym == _ss.top()) {
				_syms.push_back(sym);

				++str;
				_ss.pop();
			} else {
				auto resSym = _rules.getRes(_ss.top(), sym);

				_ss.pop();

				for (auto &res : resSym) {
					_ss.push(res);
				}
			}
		}

		return _syms;
	}

private:
	std::stack<const Symbol *> _ss;
	const SymbolRuleTable &_rules;
};

int main(int argc, char **argv)
{
	if (argc < 2) {
		std::cout << "usage:\n\tll '(a+a)'" << std::endl;
		return 0;
	}

	Lexer lexer(srt);

	std::vector<const Symbol *> syms = lexer.parse(argv[1]);

	std::cout << "finished parsing" << std::endl;

	for (auto &i : syms) {
		std::cout << i->name << std::endl;
	}

	return 0;
}

import unittest
from precalc import evaluator, parser

class TestImports(unittest.TestCase):    
    def test_imports(self):
        self.assertEqual(evaluator.__name__, "precalc.evaluator")
        self.assertEqual(parser.__name__, "precalc.parser")

class TestTokenizer(unittest.TestCase):
    def setUp(self):
        self.tokenizer = parser.Parser()

    def test_simple_expressions(self):
        cases = ["2 + 2", "5 * 4 + 9", "9 / 10"]
        results = [["2", "+", "2"],
                   ["5", "*", "4", "+", "9"],
                   ["9", "/", "10"]]
        for (case, result) in zip(cases, results):
            self.tokenizer.tokenize(case)
            self.assertListEqual(self.tokenizer.tokens, result)

    def test_expressions_with_negative_values(self):
        cases = ["-1 - 1", "5 - -1", "4 + ( -1 - -1 ) * 2"]
        results = [["-1", "-", "1"],
                   ["5", "-", "-1"],
                   ["4", "+", "(", "-1", "-", "-1", ")", "*", "2"]]
        for (case, result) in zip(cases, results):
            self.tokenizer.tokenize(case)
            self.assertListEqual(self.tokenizer.tokens, result)

if __name__ == "__main__":
    unittest.main()

import unittest
from precalc import evaluator, parser

class TestImports(unittest.TestCase):
    
    def test_imports(self):
        self.assertEqual(evaluator.__name__, "precalc.evaluator")
        self.assertEqual(parser.__name__, "precalc.parser")


if __name__ == "__main__":
    unittest.main()

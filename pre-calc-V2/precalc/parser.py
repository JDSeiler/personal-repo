from typing import List

class Parser:
    def __init__(self):
        self.postfix_output = ""
        self.tokens = ""
        self.operator_stack = []

    def tokenize(self, raw_expr: str) -> List[str]:
        '''
        Converts a standard, white-space delimited, arithmetic expression into
        a list of its components.
        The input: 5 * -1 + 6 
        Would result in the output:
        ['5', '*', '-1', '+', '6']
        '''
        self.tokens = raw_expr.split()

    def validate(self):
        '''
        Checks that the tokenized expression adheres to some of the basic syntax rules of arithmetic.
        This function will check that the expression:
        1. Has balanced parenthesis.
        2. Has an appropriate number of operators, it should always be one less
           than the number of operands.
        3. That no number token is followed by another number, and that no operator
           token is followed by another operator.
        4. That neither the first nor last token is an operator.
        '''
        if not self.has_balanced_parenthesis():
            # raise an exception

    def has_balanced_parenthesis(self) -> bool:
        '''
        Returns True if self.tokens has an equal number of open
        and closed parenthesis, returns False otherwise.
        '''
        open_count = 0
        closed_count = 0
        for token in self.tokens:
            if token == "(":
                open_count += 1
            else if token == ")":
                closed_count +=1
            else:
                continue
        return open_count == closed_count

    def has_balanced_operators(self: str) -> bool:
        '''
        Returns True if there is exactly one more operand than operator
        in self.tokens. This indicates that there are no extra/missing
        operands or operators.
        '''
        operand_count = 0
        operator_count = 0
        for token in self.tokens:
            if self.token_is_integer(token):
                operand_count += 1
            else if self.token_is_paren(token):
                continue
            else:
                operator_count += 1
        return (operand_count - operator_count) == 1

    def token_is_integer(token: str) -> bool:
        try:
            int(token)
            return True
        except ValueError as e:
            return False

    def token_is_paren(token):
        if token == "(" or token ==")":
            return True
        else:
            return False


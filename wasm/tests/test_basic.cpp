/**
 * Basic arithmetic tests for Achronyme Core
 *
 * Tests the Lexer, Parser, and Evaluator with simple expressions.
 */

#include <gtest/gtest.h>
#include "../src/parser/lexer.hpp"
#include "../src/parser/parser.hpp"
#include "../src/parser/evaluator.hpp"

using namespace achronyme;

// Helper function to evaluate an expression
double eval(const std::string& expr) {
    parser::Lexer lexer(expr);
    auto tokens = lexer.tokenize();

    parser::Parser parser(tokens);
    auto ast = parser.parse();

    parser::Evaluator evaluator;
    auto result = evaluator.evaluate(ast.get());

    return result.asNumber();
}

// ============================================================================
// Lexer Tests
// ============================================================================

TEST(LexerTest, TokenizeSimpleExpression) {
    parser::Lexer lexer("2 + 3");
    auto tokens = lexer.tokenize();

    ASSERT_EQ(tokens.size(), 4); // NUMBER, PLUS, NUMBER, END
    EXPECT_EQ(tokens[0].type, parser::TokenType::NUMBER);
    EXPECT_DOUBLE_EQ(tokens[0].value, 2.0);
    EXPECT_EQ(tokens[1].type, parser::TokenType::PLUS);
    EXPECT_EQ(tokens[2].type, parser::TokenType::NUMBER);
    EXPECT_DOUBLE_EQ(tokens[2].value, 3.0);
    EXPECT_EQ(tokens[3].type, parser::TokenType::END);
}

TEST(LexerTest, TokenizeDecimalNumbers) {
    parser::Lexer lexer("3.14");
    auto tokens = lexer.tokenize();

    ASSERT_EQ(tokens.size(), 2); // NUMBER, END
    EXPECT_EQ(tokens[0].type, parser::TokenType::NUMBER);
    EXPECT_DOUBLE_EQ(tokens[0].value, 3.14);
}

TEST(LexerTest, TokenizeScientificNotation) {
    parser::Lexer lexer("1e-3");
    auto tokens = lexer.tokenize();

    ASSERT_EQ(tokens.size(), 2); // NUMBER, END
    EXPECT_EQ(tokens[0].type, parser::TokenType::NUMBER);
    EXPECT_DOUBLE_EQ(tokens[0].value, 0.001);
}

// ============================================================================
// Basic Arithmetic Tests
// ============================================================================

TEST(EvaluatorTest, Addition) {
    EXPECT_DOUBLE_EQ(eval("2 + 3"), 5.0);
    EXPECT_DOUBLE_EQ(eval("10 + 5"), 15.0);
}

TEST(EvaluatorTest, Subtraction) {
    EXPECT_DOUBLE_EQ(eval("5 - 3"), 2.0);
    EXPECT_DOUBLE_EQ(eval("10 - 15"), -5.0);
}

TEST(EvaluatorTest, Multiplication) {
    EXPECT_DOUBLE_EQ(eval("2 * 3"), 6.0);
    EXPECT_DOUBLE_EQ(eval("5 * 7"), 35.0);
}

TEST(EvaluatorTest, Division) {
    EXPECT_DOUBLE_EQ(eval("6 / 2"), 3.0);
    EXPECT_DOUBLE_EQ(eval("10 / 4"), 2.5);
}

TEST(EvaluatorTest, Power) {
    EXPECT_DOUBLE_EQ(eval("2 ^ 3"), 8.0);
    EXPECT_DOUBLE_EQ(eval("5 ^ 2"), 25.0);
}

// ============================================================================
// Precedence Tests
// ============================================================================

TEST(PrecedenceTest, MultiplicationBeforeAddition) {
    EXPECT_DOUBLE_EQ(eval("2 + 3 * 4"), 14.0); // 2 + (3 * 4) = 14
    EXPECT_DOUBLE_EQ(eval("5 * 2 + 3"), 13.0); // (5 * 2) + 3 = 13
}

TEST(PrecedenceTest, DivisionBeforeSubtraction) {
    EXPECT_DOUBLE_EQ(eval("10 - 6 / 2"), 7.0); // 10 - (6 / 2) = 7
}

TEST(PrecedenceTest, PowerBeforeMultiplication) {
    EXPECT_DOUBLE_EQ(eval("2 * 3 ^ 2"), 18.0); // 2 * (3 ^ 2) = 18
}

TEST(PrecedenceTest, RightAssociativePower) {
    EXPECT_DOUBLE_EQ(eval("2 ^ 3 ^ 2"), 512.0); // 2 ^ (3 ^ 2) = 2 ^ 9 = 512
}

// ============================================================================
// Parentheses Tests
// ============================================================================

TEST(ParenthesesTest, OverridePrecedence) {
    EXPECT_DOUBLE_EQ(eval("(2 + 3) * 4"), 20.0);
    EXPECT_DOUBLE_EQ(eval("2 * (3 + 4)"), 14.0);
}

TEST(ParenthesesTest, Nested) {
    EXPECT_DOUBLE_EQ(eval("((2 + 3) * 4)"), 20.0);
    EXPECT_DOUBLE_EQ(eval("2 * ((3 + 4) * 5)"), 70.0);
}

// ============================================================================
// Unary Minus Tests
// ============================================================================

TEST(UnaryMinusTest, NegateNumber) {
    EXPECT_DOUBLE_EQ(eval("-5"), -5.0);
    EXPECT_DOUBLE_EQ(eval("-3.14"), -3.14);
}

TEST(UnaryMinusTest, DoubleNegation) {
    EXPECT_DOUBLE_EQ(eval("--5"), 5.0); // -(-5) = 5
}

TEST(UnaryMinusTest, InExpression) {
    EXPECT_DOUBLE_EQ(eval("-5 + 3"), -2.0);
    EXPECT_DOUBLE_EQ(eval("2 * -3"), -6.0);
}

// ============================================================================
// Complex Expressions
// ============================================================================

TEST(ComplexTest, MixedOperations) {
    EXPECT_DOUBLE_EQ(eval("2 + 3 * 4 - 5"), 9.0);
    EXPECT_DOUBLE_EQ(eval("10 / 2 + 3 * 4"), 17.0);
}

TEST(ComplexTest, WithParenthesesAndPower) {
    EXPECT_DOUBLE_EQ(eval("(2 + 3) ^ 2"), 25.0);
    EXPECT_DOUBLE_EQ(eval("2 ^ (3 + 1)"), 16.0);
}

// ============================================================================
// Decimal and Scientific Notation Tests
// ============================================================================

TEST(DecimalTest, BasicDecimal) {
    EXPECT_DOUBLE_EQ(eval("3.14 * 2"), 6.28);
    EXPECT_NEAR(eval("0.1 + 0.2"), 0.3, 1e-10); // Floating point precision
}

TEST(ScientificTest, BasicScientific) {
    EXPECT_DOUBLE_EQ(eval("1e3"), 1000.0);
    EXPECT_DOUBLE_EQ(eval("1e-3"), 0.001);
    EXPECT_DOUBLE_EQ(eval("2.5e2"), 250.0);
}

// ============================================================================
// Error Cases (should be added in Phase 2 with proper error handling)
// ============================================================================

// TODO: Add tests for division by zero, syntax errors, etc.

// Main
int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}

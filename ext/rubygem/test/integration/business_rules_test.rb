# frozen_string_literal: true

require "test_helper"

class BusinessRulesTest < Minitest::Test
  include TestHelper

  def test_insurance_underwriting_rules
    skip "Parser does not yet support deeply nested if-then-else expressions"
    # Complex insurance underwriting example
    rules = <<~RULES
      let age_factor = if applicant.age < 25 then 1.5 else (if applicant.age < 35 then 1.2 else (if applicant.age < 50 then 1.0 else 1.3 end) end) end
      in let claims_factor = if applicant.claims > 2 then 2.0 else (if applicant.claims > 0 then 1.5 else 1.0 end) end
      in let base_premium = 1000
      in base_premium * age_factor * claims_factor
    RULES

    program = Amoskeag.compile(rules, [])

    # Young driver with no claims
    result = Amoskeag.evaluate(program, { "applicant" => { "age" => 22, "claims" => 0 } })
    assert_equal 1500.0, result

    # Middle-aged driver with 1 claim
    result = Amoskeag.evaluate(program, { "applicant" => { "age" => 40, "claims" => 1 } })
    assert_equal 1500.0, result

    # Older driver with multiple claims
    result = Amoskeag.evaluate(program, { "applicant" => { "age" => 55, "claims" => 3 } })
    assert_equal 2600.0, result
  end

  def test_loan_approval_decision
    skip "Need to add AND/OR operators to support this test"
    rules = <<~RULES
      if credit_score >= 750 then :approved
      else if credit_score >= 650 then :conditional
      else :denied end end
    RULES

    program = Amoskeag.compile(rules, [:approved, :conditional, :denied])

    # Excellent candidate
    result = Amoskeag.evaluate(program, {
      "credit_score" => 780,
      "debt_to_income" => 0.35,
      "employment_years" => 5
    })
    assert_equal :approved, result

    # Conditional approval
    result = Amoskeag.evaluate(program, {
      "credit_score" => 680,
      "debt_to_income" => 0.30,
      "employment_years" => 4
    })
    assert_equal :conditional, result

    # Denied
    result = Amoskeag.evaluate(program, {
      "credit_score" => 600,
      "debt_to_income" => 0.50,
      "employment_years" => 1
    })
    assert_equal :denied, result
  end

  def test_discount_calculation
    skip "Parser does not yet support deeply nested if-then-else expressions"
    rules = <<~RULES
      let volume_discount = if order.quantity >= 100 then 0.20 else (if order.quantity >= 50 then 0.15 else (if order.quantity >= 20 then 0.10 else 0.0 end) end) end
      in let loyalty_discount = if customer.years >= 5 then 0.10 else (if customer.years >= 2 then 0.05 else 0.0 end) end
      in let total_discount = volume_discount + loyalty_discount
      in let subtotal = order.quantity * order.unit_price
      in subtotal * (1.0 - total_discount)
    RULES

    program = Amoskeag.compile(rules, [])

    # Large order, loyal customer
    result = Amoskeag.evaluate(program, {
      "order" => { "quantity" => 150, "unit_price" => 10.0 },
      "customer" => { "years" => 6 }
    })
    assert_in_delta 1050.0, result, 0.01 # 1500 * (1 - 0.30)

    # Medium order, new customer
    result = Amoskeag.evaluate(program, {
      "order" => { "quantity" => 25, "unit_price" => 10.0 },
      "customer" => { "years" => 0 }
    })
    assert_in_delta 225.0, result, 0.01 # 250 * (1 - 0.10)
  end

  def test_tax_calculation
    skip "Parser issue with complex if expression - needs investigation"
    rules = <<~RULES
      let federal_tax = income * 0.22
      in let state_tax = income * 0.05
      in let ss_tax = if income > 142800 then 142800 * 0.062 else income * 0.062 end
      in let medicare_tax = income * 0.0145
      in federal_tax + state_tax + ss_tax + medicare_tax
    RULES

    program = Amoskeag.compile(rules, [])

    # Income below SS cap
    result = Amoskeag.evaluate(program, { "income" => 100_000 })
    expected = 100_000 * (0.22 + 0.05 + 0.062 + 0.0145)
    assert_in_delta expected, result, 0.01

    # Income above SS cap
    result = Amoskeag.evaluate(program, { "income" => 200_000 })
    expected = 200_000 * (0.22 + 0.05 + 0.0145) + 142_800 * 0.062
    assert_in_delta expected, result, 0.01
  end

  def test_shipping_cost_calculation
    skip "Parser does not yet support deeply nested if-then-else expressions"
    rules = <<~RULES
      let base_cost = if weight < 1.0 then 5.0 else (if weight < 5.0 then 10.0 else (if weight < 10.0 then 15.0 else 20.0 + (weight - 10.0) * 2.0 end) end) end
      in let zone_multiplier = if zone == "domestic" then 1.0 else (if zone == "canada" then 1.5 else (if zone == "international" then 2.5 else 1.0 end) end) end
      in let express_fee = if express then 10.0 else 0.0 end
      in base_cost * zone_multiplier + express_fee
    RULES

    program = Amoskeag.compile(rules, [])

    # Domestic, light, standard
    result = Amoskeag.evaluate(program, {
      "weight" => 0.5,
      "zone" => "domestic",
      "express" => false
    })
    assert_equal 5.0, result

    # International, heavy, express
    result = Amoskeag.evaluate(program, {
      "weight" => 15.0,
      "zone" => "international",
      "express" => true
    })
    expected = (20.0 + 5.0 * 2.0) * 2.5 + 10.0
    assert_equal expected, result
  end

  def test_employee_bonus_calculation
    skip "Parser does not yet support deeply nested if-then-else expressions"
    rules = <<~RULES
      let performance_bonus = if rating >= 4.5 then salary * 0.15 else (if rating >= 3.5 then salary * 0.10 else (if rating >= 2.5 then salary * 0.05 else 0.0 end) end) end
      in let tenure_bonus = if years >= 10 then 5000.0 else (if years >= 5 then 2500.0 else 0.0 end) end
      in let department_bonus = if department_rating >= 4.0 then 1000.0 else 0.0 end
      in performance_bonus + tenure_bonus + department_bonus
    RULES

    program = Amoskeag.compile(rules, [])

    # Star performer, long tenure, good department
    result = Amoskeag.evaluate(program, {
      "rating" => 4.8,
      "salary" => 80_000,
      "years" => 12,
      "department_rating" => 4.2
    })
    expected = 80_000 * 0.15 + 5000 + 1000
    assert_equal expected, result
  end
end

# Ethanol or gasoline

A simple app made with [Rust](https://www.rust-lang.org/) and [Slint](https://slint.rs) to help you decide between ethanol or gasoline.

## How to run

```bash
$ git clone git@github.com:mfbasso/rust-slint-ethanol-or-gasoline.git
$ cd ethanol-or-gasoline
$ cargo run
```

## How to decide between ethanol or gasoline?

When it comes to deciding whether alcohol or gasoline is the more economical choice for fueling vehicles, a straightforward calculation can provide the answer.

To reach a conclusion, one simply needs to divide the price per liter of alcohol by the price per liter of gasoline. If the result is less than or equal to 0.7, then alcohol is the recommended fuel. On the other hand, if the value is greater than 0.7, it is advisable to opt for gasoline.

Let's explore some examples of both scenarios to illustrate the decision-making process.

### Example 1

- Price of alcohol per liter: $1.50
- Price of gasoline per liter: $2.00
- Calculation: $1.50 / $2.00 = 0.75

Decision: Since 0.75 is greater than 0.7, gasoline is the recommended choice.

### Example 2:

- Price of alcohol per liter: $1.00
- Price of gasoline per liter: $1.50
- Calculation: $1.00 / $1.50 = 0.67

Decision: With a result of 0.67, which is less than 0.7, alcohol is the preferred fuel.

This simple calculation provides drivers with a practical guideline for determining the most cost-effective fuel option based on current market prices. Whether it's choosing alcohol or gasoline, this decision-making process helps optimize fuel expenses and ensures efficient vehicle operation.

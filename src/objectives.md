# Objectives
- Variables
Let's go section variables https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html and implement a few features that our app will need. When I say app let's assume that it's an application not an Mobile app.

Let's start with basic app structure, we will need following details from user for one subscription
Subscription title, price, duration (weekly, monthly, yearly), start date
So we will need four basic variables, let's code

Screenshot 01-variable

I am storing price as 999 instead of 9.99, I will explain that later why I did stored it as 999 instead of 9.99 in later sections.


We want the price to be printed as $ actual price entered / 100

Screenshot 02-variable

We are trying to divde a string with an integer. 
This says that number should be stored as integer if you want to make arithmatic operations on them

The output is showing as
Your subscription for Github at $9 per monthly starting 14/04/2026
why not 9.99 because we did not specified the type of the variable dollar_price it defaulting to integer
Section https://doc.rust-lang.org/book/ch03-02-data-types.html

Screenshot 03

New error, now the price needs to be floating point, but we have decided not to have floating point for price.
Screenshot 04

There is no documentation on how to divide a integer without converting it to float. 
Compiler gives the message
```
For more information about this error, try `rustc --explain E0308`.
```
Screenshot 5

This is also not helping us to achieve what we want to do.
Let's search on Google we need a way to change use the integer as float so that we can make the calculations. This is called as variable casting so let's search for variable casting in rust https://doc.rust-lang.org/rust-by-example/types/cast.html
So the keyword is `as` let's try this

screenshot 6

So far so good now we are running next problem f32 / integer
Which integer? 100 it's an integer so we need to convert this to float

Screenshot 7 and 8 same result

So we have covered basic variable and casting.

Next up we will get do conditional logic and calculate the yearly price for the subscription.

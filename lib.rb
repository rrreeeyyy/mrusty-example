def fib(n, a = 1, b = 0)
  if n == 0
    0
  elsif n == 1
    a
  else
    fib(n - 1, a + b, a)
  end
end

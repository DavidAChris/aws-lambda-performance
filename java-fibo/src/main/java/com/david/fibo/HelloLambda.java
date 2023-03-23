package com.david.fibo;

public class HelloLambda {
    public int handleRequest(int event) {
        return fib_recur(event);
    }

    static int fib_recur(int num) {
        if (num <= 1) {
            return num;
        } else {
            return fib_recur(num - 1) + fib_recur(num - 2);
        }
    }
}

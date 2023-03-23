package com.david.fibo;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class HelloLambdaTest {
    @Test
    void shouldReturnHelloMessage() {
        HelloLambda sut = new HelloLambda();
        Assertions.assertEquals(1, sut.handleRequest(1));
    }
}
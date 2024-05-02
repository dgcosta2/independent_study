package com.craftinginterpreters.lox;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Scanner;

public class Tester {

    public static void test(String path) throws IOException {
        File fp = new File("../buffer/buffer.txt");
        String fileNumber = path.substring(path.lastIndexOf('t') + 1,
                path.lastIndexOf('.'));
        Scanner goldenFileScanner = new Scanner(new File("../golden/test" + fileNumber + ".golden"));
        // holds a reference to 'System.out'
        PrintStream sysOut = System.out;
        PrintStream newOutput = new PrintStream(fp);
        Scanner fileIn = new Scanner(fp);
        System.setOut(newOutput);

        StringBuilder valueBuilder = new StringBuilder();
        while (goldenFileScanner.hasNext()) {
            valueBuilder.append(goldenFileScanner.nextLine());
        }
        String value = valueBuilder.toString();

        Lox.runFile(path);

        System.setOut(sysOut);
        StringBuilder actualOutputBuilder = new StringBuilder();
        while (fileIn.hasNext()) {
            actualOutputBuilder.append(fileIn.nextLine());
        }
        String actualOutput = actualOutputBuilder.toString();
        if (!value.equals(actualOutput)) {
            throw new TesterException("File " + path + "\tNOT OK");
        }
    }
}

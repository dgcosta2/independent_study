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

        String value = goldenFileScanner.nextLine().strip();

        Lox.runFile(path);

        System.setOut(sysOut);
        String actualOutput = fileIn.nextLine();
        if (!value.equals(actualOutput)) {
            throw new TesterException("File " + path + "\tNOT OK");
        }
    }
}

package ch02.B07;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int t = sc.nextInt();
        int n = sc.nextInt();

        int[] employees = new int[t+1];
        for (int i=0; i<n; i++) {
            int start = sc.nextInt();
            int end = sc.nextInt();
            employees[start]++;
            employees[end]--;
        }

        for (int i=0; i<=t-1; i++) {
            if (i > 0) { employees[i] += employees[i-1]; }
            System.out.println(employees[i]);
        }
    }
}

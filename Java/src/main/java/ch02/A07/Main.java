package ch02.A07;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int d = sc.nextInt();
        int n = sc.nextInt();

        int[] attendance = new int[d+2];
        for (int i=0; i<n; i++) {
            int left = sc.nextInt();
            int right = sc.nextInt();
            attendance[left] += 1;
            attendance[right+1] -= 1;
        }
        for (int i=1; i<=d; i++) {
            attendance[i] += attendance[i-1];
            System.out.println(attendance[i]);
        }


    }
}

package ch03.B12;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        double n = (double)sc.nextInt();

        double ng = 0;
        double ok = 100;

        for (int i=0; i<100; i++) {
            double mid = (ok + ng) / 2.0;
            if (f(mid) < n) ng = mid;
            else ok = mid;
        }

        System.out.printf("%.6f\n", ok);
    }

    public static double f(double x) {
        return x*x*x + x;
    }
}

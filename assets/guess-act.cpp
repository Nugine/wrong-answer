#include <cstdio>
using namespace std;

int main(int argc, char const *argv[]) {
    int ans;
    int ask;

    FILE *in = fopen(argv[1], "r");
    fscanf(in, "%d", &ans);
    fclose(in);

    FILE *out = fopen(argv[2], "w");

    bool flag = false;

    while (scanf("%d", &ask) == 1) {
        if (ask < 1 || ask > 100) {
            break;
        }
        if (ans > ask) {
            printf(">\n");
            fflush(stdout);
        } else if (ans < ask) {
            printf("<\n");
            fflush(stdout);
        } else {
            printf("=\n");
            fflush(stdout);
            flag = true;
            break;
        }
    }

    if (flag) {
        fprintf(out, "AC\n");
    } else {
        fprintf(out, "WA\n");
    }
}
#include <cstdio>
using namespace std;

int main() {
    int l = 1, r = 101;

    char ans[8];

    while (l + 1 < r) {
        int mid = (l + r) / 2;
        printf("%d\n", mid);
        fflush(stdout);

        scanf("%s", ans);
        if (ans[0] == '<') {
            r = mid;
        } else if (ans[0] == '>') {
            l = mid + 1;
        } else {
            break;
        }
    }
}

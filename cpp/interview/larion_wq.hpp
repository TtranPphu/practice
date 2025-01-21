#include <bits/stdc++.h>
using namespace std;

namespace larion_wq {
class Solution {
 public:
  vector<int> infected(int n, vector<int> initial_infected,
                       vector<vector<int>> updates) {
    vector<int> result(n, -1);
    vector<bool> vacinated(n, false);
    vector<shared_ptr<pair<set<int>, bool>>> contacts;

    for (int i = 0; i < n; i++) {
      auto p = make_shared<pair<set<int>, bool>>(set<int>(), false);
      p->first.insert(i);
      contacts.push_back(p);
    }

    for (auto it : initial_infected) {
      result[it] = 0;
      contacts[it]->second = true;
    }

    unsigned t = 0;
    for (auto update : updates) {
      t++;
      if (update[0]) {  // vacinate
        vacinated[update[1]] = true;

      } else {  // contact
        int p1 = update[1], p2 = update[2];

        if (contacts[p1]->first.size() < contacts[p2]->first.size()) {
          swap(p1, p2);
        }
        if (contacts[p1]->first.find(p2) == contacts[p1]->first.end()) {
          {
            auto spread = [&result, &vacinated, &t](set<int>& contacts) {
              for (auto contact : contacts)
                if (!vacinated[contact] && result[contact] < 0)
                  result[contact] = t;
            };
            if (contacts[p1]->second && !contacts[p2]->second) {
              contacts[p2]->second = true;
              spread(contacts[p2]->first);
            } else if (!contacts[p1]->second && contacts[p2]->second) {
              contacts[p1]->second = true;
              spread(contacts[p1]->first);
            }
          }
          {
            auto contact_p2 = contacts[p2];
            contacts[p1]->first.insert(contacts[p2]->first.begin(),
                                       contacts[p2]->first.end());
            for (auto contact : contact_p2->first) {
              contacts[contact] = contacts[p1];
            }
          }
        }
      }
    }
    return result;
  }
};
}  // namespace larion_wq
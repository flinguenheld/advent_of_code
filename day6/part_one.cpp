#include <cstdint>
#include <fstream>
#include <iostream>
#include <vector>

typedef struct time_distance {
  int time;
  int distance;
} time_distance;

// ------------------------------------------------------------------------------------------ Line peeler ---
void peel_my_liiine(std::vector<time_distance> &tab, bool time, const std::string &txt) {

  int i = 0;
  std::string nb;
  for (auto it = txt.begin(); it != txt.end(); ++it) {

    if (std::isdigit(*it))
      nb += *it;

    if ((*it == ' ' || it == txt.end() - 1) && !nb.empty()) {

      if (time) {
        tab.push_back({});
        tab[i].time = std::stoi(nb);

      } else
        tab[i].distance = std::stoi(nb);

      nb.clear();
      ++i;
    }
  }
}

// ------------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<time_distance> tab;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {

      if (line.find("Time:") != std::string::npos) {
        peel_my_liiine(tab, true, line);

      } else {
        peel_my_liiine(tab, false, line);
      }
    }

    file.close();
  }

  // -------------------------------------------------------------------------------------- Calculations ---
  int total = 1;
  for (const auto &set : tab) {
    int nb_win = 0;
    // std::cout << "Current set: t=" << set.time << " d=" << set.distance << "  ------\n";

    for (int charge = 0; charge < set.time; ++charge) {

      // std::cout << "Charge: " << charge << "   time last: " << set.time - charge
      //           << "   distance made: " << charge * (set.time - charge) << "\n";

      if ((set.time - charge) * charge > set.distance)
        nb_win++;
    }
    std::cout << nb_win << "\n";
    total *= nb_win;
  }

  std::cout << "Total :" << total << "\n";
  return 0;
}

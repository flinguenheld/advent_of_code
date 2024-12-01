#include <array>
#include <fstream>
#include <iostream>
#include <list>
#include <vector>

struct Lentil {
  std::string txt;
  int box;
  int val;
};

// ------------------------------------------------------------------------------------------ Box num ---
int64_t box_num(const std::string &step) {
  int64_t value = 0;
  for (const auto &c : step) {
    value += (int64_t)c;
    value *= 17;
    value %= 256;
  }
  return value;
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {
  std::string line;
  std::ifstream file("input.txt");
  // std::ifstream file("example.txt");

  std::vector<Lentil> sequence;
  std::array<std::list<Lentil>, 256> boxes;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (!line.empty()) {

        Lentil lentil;

        for (auto it = line.begin(); it != line.end(); ++it) {
          if (std::isdigit(*it))
            lentil.val = *it - '0';
          else if (std::isalpha(*it))
            lentil.txt += *it;
          else if (*it == '-')
            lentil.val = -1;

          if (*it == ',' || it == line.end() - 1) {
            lentil.box = box_num(lentil.txt); // Perform here one time
            sequence.push_back(lentil);

            lentil.txt.clear();
          }
        }
      }
    }

    file.close();
  }

  // ---------------------------------------------------------------------------------------- Process ---
  for (const auto &lentil : sequence) {

    // Remove -
    if (lentil.val == -1) {
      auto it = boxes[lentil.box].begin();
      while (it != boxes[lentil.box].end()) {

        if ((*it).txt == lentil.txt)
          it = boxes[lentil.box].erase(it);
        else
          ++it;
      }

      // Add or update =
    } else {

      // Find a lens with the same txt
      auto it = boxes[lentil.box].begin();
      while (it != boxes[lentil.box].end()) {

        if ((*it).txt == lentil.txt) {
          (*it).val = lentil.val;
          break;
        } else
          ++it;
      }

      if (it == boxes[lentil.box].end())
        boxes[lentil.box].push_back(lentil);
    }
  }

  // -------------------------------------------------------------------------------------------- Add ---
  int total = 0;

  int i = 0;
  for (const auto &box : boxes) {

    if (!box.empty()) {

      int slot = 0;
      for (const auto &l : box) {
        ++slot;
        total += (l.box + 1) * slot * l.val;
      }
    }
    ++i;
  }

  std::cout << total << "\n";
  return 0;
}

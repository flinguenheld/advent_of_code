#include <fstream>
#include <iostream>
#include <vector>

// ------------------------------------------------------------------------------------------ Display ---
void display_vec(const std::vector<std::vector<int64_t>> &vec) {

  std::cout << "-------------------------------\n";
  for (const auto &line : vec) {
    for (const auto &val : line) {

      std::string nb = std::to_string(val);
      while (nb.size() < 3)
        nb = " " + nb;

      std::cout << nb << " ";
    }
    std::cout << "\n";
  }
}

// -------------------------------------------------------------------------------------- Line peeler ---
void peel(std::vector<int64_t> &line, const std::string &txt) {

  std::string nb;
  for (auto it = txt.begin(); it != txt.end(); ++it) {

    if (std::isdigit(*it) || (*it) == '-')
      nb += *it;

    if (std::isspace(*it) || it == txt.end() - 1) {
      line.emplace_back(std::stod(nb));
      nb.clear();
    }
  }
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<std::vector<int64_t>> history;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (!line.empty()) {
        history.push_back({});
        peel(history.back(), line);
      }
    }

    file.close();
  }

  // ---------------------------------------------------------------------------------------- Process ---
  int64_t total = 0;
  int64_t previous = 0;

  for (int l = 0; l < history.size(); ++l) {

    // Create all temporary lines --
    std::vector<std::vector<int64_t>> tmp;
    tmp.push_back(history[l]);
    while (true) {

      std::vector<int64_t> new_line;
      for (int i = 1; i < tmp.back().size(); ++i)
        new_line.push_back(tmp.back()[i] - tmp.back()[i - 1]);

      tmp.emplace_back(std::move(new_line));

      if ((tmp.back().front() == 0 && tmp.back().back() == 0) || tmp.back().size() == 1)
        break;
    }

    // Calculate values in the end (avoid vector push)
    // display_vec(tmp);
    previous = tmp.back().back();

    for (int i = tmp.size() - 2; i >= 0; --i)
      previous = tmp[i].back() + previous;

    // Add the last one
    total += previous;
  }

  std::cout << total << "\n";
  return 0;
}

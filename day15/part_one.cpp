#include <cstdint>
#include <fstream>
#include <iostream>
#include <vector>

// --------------------------------------------------------------------------------------------- Step ---
int64_t add_step(const std::string &step) {

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

  std::vector<std::string> sequence;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (!line.empty()) {
        sequence.push_back({});

        for (auto it = line.begin(); it != line.end(); ++it) {
          if (std::isalnum(*it) || *it == '-' || *it == '=')
            sequence.back() += *it;
          else if (*it == ',')
            sequence.push_back({});
        }
      }
    }

    file.close();
  }

  // ---------------------------------------------------------------------------------------- Process ---
  int64_t value = 0;
  for (const auto &s : sequence)
    value += add_step(s);

  std::cout << value << "\n";

  return 0;
}

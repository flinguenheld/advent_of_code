#include <cstdint>
#include <fstream>
#include <iostream>

// ------------------------------------------------------------------------------------------ Line peeler ---
int64_t peel_my_liiine(const std::string &txt) {

  std::string nb;
  for (const auto &c : txt) {
    if (std::isdigit(c))
      nb += c;
  }
  return std::stod(nb);
}

// ------------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  int64_t time;
  int64_t distance;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      if (line.find("Time:") != std::string::npos)
        time = peel_my_liiine(line);
      else
        distance = peel_my_liiine(line);
    }

    file.close();
  }

  // std::cout << "Time: " << time << "\n";
  // std::cout << "Distance: " << distance << "\n";

  // Both solutions work in less than 1 second

  // ---------------------------------------------------------------------------------------- Solution 1 ---
  // ---------------------------------------------------------------------------------- 52.144.051 loops ---
  int64_t nb_win = 0;
  for (int64_t charge = 0; charge < time; ++charge) {

    if ((time - charge) * charge > distance)
      nb_win++;
    else if (nb_win) // Avoid ~ 5 million loops but imperceptible on the 57 million
      break;
  }
  std::cout << nb_win << "\n";

  // ---------------------------------------------------------------------------------------- Solution 2 ---
  // ---------------------------------------------------------------------------------- 11.165.888 loops ---
  int64_t start = 0;
  for (; start < time; ++start) {
    if ((time - start) * start > distance)
      break;
  }

  int64_t end = time;
  for (; end > 0; --end) {
    if ((time - end) * end > distance)
      break;
  }

  std::cout << end - start + 1 << "\n";

  return 0;
}

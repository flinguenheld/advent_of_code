#include <algorithm>
#include <cctype>
#include <codecvt>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

typedef struct {
  std::string value;
  std::string key;
} Record;

// ------------------------------------------------------------------------------------------ Display ---
void _display_colors(const std::string &txt, int start) {
  for (int i = start; i < txt.size(); ++i) {
    if (txt[i] == '.')
      std::cout << "\033[1;31m.\033[0m";
    else if (txt[i] == '#')
      std::cout << "\033[1;34m#\033[0m";
    else
      std::cout << "\033[1;33m?\033[0m";
  }
}
void display_records(const std::vector<Record> &records, int start_value = 0, int start_key = 0) {
  for (const auto &r : records) {
    _display_colors(r.value, start_value);
    std::cout << "  ";
    _display_colors(r.key, start_key);
    std::cout << "\n";
  }
}

// -------------------------------------------------------------------------------------- Line peeler ---
void peel_groups(Record &r, const std::string &txt) {

  std::string nb;
  for (auto it = txt.begin(); it != txt.end(); ++it) {

    if (std::isdigit(*it))
      nb += *it;

    if (*it == ',' || it == txt.end() - 1) {
      r.key += std::string(std::stoi(nb), '#');
      if (it != txt.end() - 1)
        r.key += '.';

      nb.clear();
    }
  }
}

// -------------------------------------------------------------------------------------- Line peeler ---
std::string remove_dot(std::string txt) {

  std::string ok;

  while (txt.front() == '.' && txt.size() > 1)
    txt.erase(txt.begin());
  while (txt.back() == '.' && txt.size() > 1)
    txt.pop_back();

  for (int i = 0; i < txt.size(); ++i) {
    if (txt[i] != '.' || (txt[i] == '.' && txt[i + 1] != '.'))
      ok += txt[i];
  }

  return ok;
}

// -------------------------------------------------------------------------------------- Line peeler ---
void list_possibilities(std::string value, const std::string &key, int64_t &total, int pos) {

  int i = pos;
  for (auto it = value.begin() + pos; it != value.end(); ++it) {

    // std::cout << value << " - " << key << "    " << i << " Value sans dot: " << remove_dot(value) << "\n";

    if (*it == '?') {
      *it = '#';
      list_possibilities(value, key, total, i);

      *it = '.';
      list_possibilities(value, key, total, i);
      return;
    }
    i++;
  }
  value = remove_dot(value);

  if (value == key) {
    total++;
  }

  // std::cout << " <-----------========== OK";
  // std::cout << "OK" << txt << " - " << key << "\n";
  // std::cout << "\n";
}

// --------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<Record> records;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      Record r;
      auto space = line.find(' ');

      r.value = remove_dot(line.substr(0, space));

      peel_groups(r, line.substr(space + 1, -1));
      records.emplace_back(std::move(r));
    }

    file.close();
  }

  display_records(records);
  // ---------------------------------------------------------------------------------------- Process ---
  int64_t total = 0;
  std::vector<std::string> poss;
  for (auto &r : records) {
    list_possibilities(r.value, r.key, total, 0);
  }

  // std::cout << "\n";
  std::cout << total << "\n";

  return 0;
}

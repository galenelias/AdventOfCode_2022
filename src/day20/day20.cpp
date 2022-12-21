#include <fstream>
#include <list>
#include <string>
#include <vector>
#include <iostream>
#include <unordered_map>

int64_t sub_solve(const std::vector<int64_t>& inputs, int64_t multipler, size_t loop_count, bool should_print)
{
    std::vector<int64_t> values;
    std::vector<std::list<int64_t>::iterator> iters;

    for (auto val : inputs)
        values.push_back(val * multipler);

    std::list<int64_t> ll;
    std::list<int64_t>::iterator zero_iter;

    for (auto v : values)
    {
        auto iter = ll.insert(ll.end(), v);
        iters.push_back(iter);

        if (v == 0)
            zero_iter = iter;
    }

    if (should_print) {
        for (auto v : ll)
            std::cout << v << " ";
        std::cout << "\n";
    }

    for (size_t x = 0; x < loop_count; ++x)
    {
        for (size_t i = 0; i < values.size(); ++i)
        {
            auto v = values[i];
            auto move_to_iter = iters[i];

            if (v == 0)
                continue;

            move_to_iter = ll.erase(move_to_iter);
            if (move_to_iter == ll.end())
                move_to_iter = ll.begin();

            if (should_print)
                std::cout << "Starting iteration.  Pointing to " << *move_to_iter << "\n";

            if (v < 0) {
                auto amount = (v * -1) % ll.size();
                for (int64_t i = 0; i < amount; ++i) {
                    if (move_to_iter == ll.begin())
                        move_to_iter = ll.end();
                    --move_to_iter;
                }
                if (move_to_iter == ll.begin())
                    move_to_iter = ll.end();
            } else if (v > 0) {
                auto amount = v % ll.size();
                for (int64_t i = 0; i < amount; ++i) {
                    ++move_to_iter;
                    if (move_to_iter == ll.end())
                        move_to_iter = ll.begin();
                }
            }

            if (should_print)
                std::cout << "Moving " << v << ", before " << *move_to_iter << "\n";
            iters[i] = ll.insert(move_to_iter, v);

            // if (should_print) {
            //     std::cout << v << ": ";
            //     for (auto v : ll)
            //         std::cout << v << " ";
            //     std::cout << "\n";
            // }
        }

        if (should_print) {
            for (auto v : ll)
                std::cout << v << " ";
            std::cout << "\n";
        }
    }

    auto iter = zero_iter;
    std::cout << "*iter = " << *iter << "\n";
    int64_t result = 0;

    for (int64_t i = 0; i < 3001; ++i) {
        if (i > 0 && (i % 1000) == 0) {
            std::cout << "*iter = " << *iter << "\n";
            result += *iter;
        }

        ++iter;
        if (iter == ll.end())
            iter = ll.begin();
    }

    return result;
}

int main(int argc, char** argv)
{
    constexpr int64_t c_decryption_key = 811589153;

    bool should_print = false;
    const char* pszFilename = "day20.txt";

    if (argc > 1)
        pszFilename = argv[1];

    if (argc > 2)
        should_print = true;

    std::ifstream ifs (pszFilename, std::ifstream::in);

    std::vector<int64_t> inputs;
    int64_t val;
    while (ifs >> val) {
        inputs.push_back(val);
    }

    const int part1 = sub_solve(inputs, 1, 1, should_print);
    const int part2 = sub_solve(inputs, c_decryption_key, 10, should_print);

    std::cout << "Part 1: " << part1 << "\n";
    std::cout << "Part 2: " << part2 << "\n";
}
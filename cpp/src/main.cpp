#include <cstdint>
#include <random>
#include <unordered_map>

#include <benchmark/benchmark.h>

using KeyT = int32_t;
using ValueT = int32_t;

thread_local static std::random_device rd;
thread_local static std::mt19937 thread_rng(rd());

template<typename K, typename V> using HashMapT = std::unordered_map<K, V>;

static auto fill_linear_n(const int32_t n) -> HashMapT<KeyT, ValueT>
{
  HashMapT<KeyT, ValueT> hm{};

  for (int32_t i = 0; i < n; ++i)
  {
    hm.insert({static_cast<KeyT>(i), static_cast<ValueT>(i)});
  }

  return hm;
}

static auto fill_linear_n_lookup_one(const int32_t n) -> ValueT
{
  return fill_linear_n(n).find(static_cast<KeyT>(n / 2))->second;
}

static auto fill_linear_n_lookup_all(const int32_t n) -> int32_t
{
  const auto hm = fill_linear_n(n);

  int32_t ret = 0;
  for (int32_t i = 0; i < n; ++i)
  {
    const auto it = hm.find(static_cast<KeyT>(i));
    if (it != std::end(hm))
    {
      ++ret;
    }
  }
  return ret;
}

static auto fill_linear_n_insert_random(const int32_t n) -> int32_t
{
  auto hm = fill_linear_n(n);
  std::uniform_int_distribution<int32_t> dis(0, n);

  int32_t ret = 0;
  for (int32_t i = 0; i < n; ++i)
  {
    const auto rand_int = dis(thread_rng);
    hm.insert({static_cast<KeyT>(rand_int), static_cast<ValueT>(i)});
    if (dis(thread_rng) < (n / 2))
    {
      ++ret;
    }
  }
  return ret + static_cast<int32_t>(hm.size());
}

static auto fill_linear_n_lookup_random(const int32_t n) -> int32_t
{
  const auto hm = fill_linear_n(n);
  std::uniform_int_distribution<int32_t> dis(0, n);

  int32_t ret = 0;
  for (int32_t i = 0; i < n; ++i)
  {
    const auto it = hm.find(dis(thread_rng));
    if (it == std::end(hm))
    {
      ++ret;
    }
  }
  return ret;
}

static auto fill_linear_n_lookup_missing(const int32_t n) -> int32_t
{
  const auto hm = fill_linear_n(n);
  std::uniform_int_distribution<int32_t> dis(n, n*2);

  int32_t ret = 0;
  for (int32_t i = 0; i < n; ++i)
  {
    const auto it = hm.find(dis(thread_rng));
    if (it == std::end(hm))
    {
      ++ret;
    }
  }
  return ret;
}

static auto random_gen_only(const int32_t n) -> int32_t
{
  std::uniform_int_distribution<int32_t> dis(0, n);

  int32_t ret = 0;
  for (int32_t i = 0; i < n; ++i)
  {
    if (dis(thread_rng) < (n / 2))
    {
      ++ret;
    }
  }
  return ret;
}

static auto fill_linear_n_copy_element_wise(const int32_t n) -> int32_t
{
  const auto hm = fill_linear_n(n);
  HashMapT<KeyT, ValueT> hm_copy{};

  for (const auto [key, val] : hm)
  {
    hm_copy.insert({key, val});
  }
  return static_cast<int32_t>(hm_copy.size());
}

static auto fill_linear_n_traversal(const int32_t n) -> int32_t
{
  const auto hm = fill_linear_n(n);

  int32_t ret = 0;
  for (const auto [key, value] : hm)
  {
    ++ret;
  }
  return ret;
}

#define MAKE_BENCHMARK(name, func) \
static void name(benchmark::State& state) { \
  for (auto _ : state) \
  { \
    benchmark::DoNotOptimize(func(state.range(0))); \
  } \
} \
BENCHMARK(name)->RangeMultiplier(10)->Range(10, 100'000);

MAKE_BENCHMARK(copy_element_wise, fill_linear_n_copy_element_wise)
MAKE_BENCHMARK(fill_only, fill_linear_n)
MAKE_BENCHMARK(insert_random, fill_linear_n_insert_random)
MAKE_BENCHMARK(lookup_all, fill_linear_n_lookup_all)
MAKE_BENCHMARK(lookup_missing, fill_linear_n_lookup_missing)
MAKE_BENCHMARK(lookup_one, fill_linear_n_lookup_one)
MAKE_BENCHMARK(lookup_random, fill_linear_n_lookup_random)
MAKE_BENCHMARK(random_gen, random_gen_only)
MAKE_BENCHMARK(traversal, fill_linear_n_traversal)

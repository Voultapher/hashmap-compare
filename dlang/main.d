import std.conv : to;
import std.datetime.stopwatch : benchmark;
import std.random : uniform;
import std.range : repeat, join;
import std.stdio : writefln;

alias HashMapT(K, V) = V[K];

alias KeyT = int;
alias ValueT = int;

HashMapT!(KeyT, ValueT) fill_linear_n(const int n)
{
    HashMapT!(KeyT, ValueT) hm;

    for (int i; i < n; ++i)
    {
        hm[cast(KeyT)(i)] = cast(ValueT)(i);
    }

    return hm;
}

ValueT fill_linear_n_lookup_one(const int n)
{
    return fill_linear_n(n)[cast(KeyT)(n / 2)];
}

int fill_linear_n_lookup_all(const int n)
{
    const auto hm = fill_linear_n(n);

    int side_effect = 0;
    for (int i = 0; i < n; ++i)
    {
        const auto it = cast(KeyT)(i) in hm;
        if (it !is null)
        {
            ++side_effect;
        }
    }
    return side_effect;
}

int fill_linear_n_insert_random(const int n)
{
    auto hm = fill_linear_n(n);

    int side_effect = 0;
    for (int i = 0; i < n; ++i)
    {
        const auto rand_int = uniform(0, n);
        hm[cast(KeyT)(rand_int)] = cast(ValueT)(i);
        if (uniform(0, n) < (n / 2))
        {
            ++side_effect;
        }
    }
    return side_effect + cast(int)(hm.length);
}

int fill_linear_n_lookup_random(const int n)
{
    const auto hm = fill_linear_n(n);

    int side_effect = 0;
    for (int i = 0; i < n; ++i)
    {
        const auto it = uniform(0, n) in hm;
        if (it !is null)
        {
            ++side_effect;
        }
    }
    return side_effect;
}

int fill_linear_n_lookup_missing(const int n)
{
    const auto hm = fill_linear_n(n);

    int side_effect = 0;
    for (int i = 0; i < n; ++i)
    {
        const auto it = uniform(n, n*2) in hm;
        if (it !is null)
        {
            ++side_effect;
        }
    }
    return side_effect;
}

int random_gen_only(const int n)
{
    int side_effect = 0;
    for (int i = 0; i < n; ++i)
    {
        if (uniform(0, n) < (n / 2))
        {
            ++side_effect;
        }
    }
    return side_effect;
}

int fill_linear_n_copy_element_wise(const int n)
{
    const auto hm = fill_linear_n(n);
    HashMapT!(KeyT, ValueT) hm_copy;

    foreach (key, value; hm)
    {
        hm_copy[key] = value;
    }
    return cast(int)(hm_copy.length);
}

int fill_linear_n_traversal(const int n)
{
    const auto hm = fill_linear_n(n);

    int side_effect = 0;
    foreach (key, value; hm)
    {
        ++side_effect;
    }
    return side_effect;
}

void format_result(string name, long full_time, int size, int repetition_count)
{
    const auto average_time = full_time / repetition_count;
    const auto padding_name = 33 - (name.length + size.to!string.length);
    const auto padding_time = 13 - (average_time.to!string.length);
    writefln(
        "%s/%s%s%d ns%s%d",
        name,
        size,
        " ".repeat(padding_name).join(),
        average_time,
        " ".repeat(padding_time).join(),
        repetition_count
    );
}

void DoNotOptimize(T)(T function(const int) fn_ptr, const int size)
{
    // Avoid optimizing out code, pin return value as side effect,
    // should make usage of return value opaque to optimizer,
    // and so avoid removing 'useless' code.
    // The function implementation should still be visible to the optimizer.
    asm
    {
        mov EDI, size;
        call fn_ptr;
    }
}

void benchmark_one(T)(
    T function(const int) func,
    string name,
    int size,
    int repetition_count
)
{
    void fn_bench() { DoNotOptimize(func, size); }
    const auto result = benchmark!(fn_bench)(repetition_count);
    format_result(name, result[0].total!"nsecs", size, repetition_count);
}

void benchmark_size(int size, int repetition_count)
{
    const auto rc = repetition_count;
    benchmark_one(&fill_linear_n, "fill_only", size, rc);
    benchmark_one(&fill_linear_n_lookup_one, "lookup_one", size, rc);
    benchmark_one(&fill_linear_n_lookup_all, "lookup_all", size, rc);
    benchmark_one(&fill_linear_n_lookup_missing, "lookup_missing", size, rc);
    benchmark_one(&fill_linear_n_lookup_random, "lookup_random", size, rc);
    benchmark_one(&fill_linear_n_insert_random, "insert_random", size, rc);
    benchmark_one(&fill_linear_n_traversal, "traversal", size, rc);
    benchmark_one(&fill_linear_n_copy_element_wise, "copy_element_wise", size, rc);
    benchmark_one(&random_gen_only, "random_gen", size, rc);

    writefln("");
}

void benchmark()
{
    writefln(
"-------------------------------------------------------------
Benchmark                         Time            Iterations
-------------------------------------------------------------"
    );

    // Repetition counts are conservative guesses based on running the cpp benchmark
    benchmark_size(100_000, 150);
    benchmark_size(10_000, 1_000);
    benchmark_size(1_000, 10_000);
    benchmark_size(100, 200_000);
    benchmark_size(10, 2_000_000);

}

int main()
{
    const n = 100;
    const auto hm = fill_linear_n(n);
    assert(hm.length == n);

    const auto value = fill_linear_n_lookup_one(n);
    assert(value == (n / 2));

    const auto side_effect_a = fill_linear_n_lookup_all(n);
    assert(side_effect_a == n);

    const auto side_effect_b = fill_linear_n_insert_random(n);
    assert(side_effect_b < (n * 2));

    const auto side_effect_c = fill_linear_n_lookup_random(n);
    assert(side_effect_c == n);

    const auto side_effect_d = fill_linear_n_lookup_missing(n);
    assert(side_effect_d == 0);

    const auto side_effect_e = random_gen_only(n);
    assert(side_effect_e < n);

    const auto side_effect_f = fill_linear_n_copy_element_wise(n);
    assert(side_effect_f == n);

    const auto side_effect_g = fill_linear_n_traversal(n);
    assert(side_effect_g == n);

    benchmark();

    return 0;
}

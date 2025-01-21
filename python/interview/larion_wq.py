def sentenses(s: str, d: set) -> list:
    result = list()
    for i in range(len(s)):
        first = s[: i + 1]
        if first in d:
            rest = sentenses(s[i + 1 :], d)
            if not rest:
                result.append(first)
            else:
                for sentense in rest:
                    result.append(first + " " + sentense)
    return result

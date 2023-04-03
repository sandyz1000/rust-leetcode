"""
207. Course Schedule
https://leetcode.com/problems/course-schedule/description/

There are a total of numCourses courses you have to take, labeled from 0 to
numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi]
indicates that you must take course bi first if you want to take course ai.

For example, the pair [0, 1], indicates that to take course 0 you have to first take
course 1.

Return true if you can finish all courses. Otherwise, return false.

Example 1:
----------
Input: numCourses = 2, prerequisites = [[1,0]]
Output: true
Explanation: There are a total of 2 courses to take.
To take course 1 you should have finished course 0. So it is possible.

Example 2:
----------
Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
Output: false
Explanation: There are a total of 2 courses to take. 
To take course 1 you should have finished course 0, and to take course 0 you should
also have finished course 1. So it is impossible.


Constraints:
------------
1 <= numCourses <= 2000
0 <= prerequisites.length <= 5000
prerequisites[i].length == 2
0 <= ai, bi < numCourses
All the pairs prerequisites[i] are unique.

"""
from typing import List, Set, Dict
from collections import defaultdict


def canFinish(numCourses: int, prerequisites: List) -> bool:
    # Build graph and dfs
    visited = [0] * numCourses
    graph = defaultdict(list)
    for n1, n2 in prerequisites:
        graph[n1].append(n2)

    # Iterate for every disjoint node
    for i in range(numCourses):
        if visited[i] == 0 and not dfs(i, graph, visited):
            return False

    return True


def dfs(node: int, graph: Dict[int, List[int]], visited: Set) -> bool:
    """
    - For dfs Mark node visited[node] = -1 to look for cycle
    - Time complexity: O(V + E)  # For directed edge
    """
    # If cycle detected
    if visited[node] == -1:
        return False

    # if it is done visted, then do not visit again
    if visited[node] == 1:
        return True

    visited[node] = -1

    for child in graph[node]:
        if not dfs(child, graph, visited):
            return False

    visited[node] = 1
    return True


def cli_main():
    numCourses = 2
    prerequisites = [[1, 0]]
    ans = canFinish(numCourses, prerequisites)
    assert ans

    numCourses = 2
    prerequisites = [[1, 0], [0, 1]]
    ans = canFinish(numCourses, prerequisites)
    assert not ans


if __name__ == "__main__":
    cli_main()

/// Chapter 17: batuta - Workflow Orchestration
///
/// **CLAIM:** batuta provides deterministic workflow execution:
/// - Reproducible task ordering
/// - Type-safe DAG execution
/// - Deterministic scheduling
///
/// **VALIDATION:** `make run-ch17`
use anyhow::Result;
use std::collections::{HashMap, VecDeque};

/// Task status
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Workflow task
#[derive(Debug, Clone)]
struct Task {
    id: String,
    dependencies: Vec<String>,
    status: TaskStatus,
}

impl Task {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            dependencies: Vec::new(),
            status: TaskStatus::Pending,
        }
    }

    fn depends_on(mut self, dep: &str) -> Self {
        self.dependencies.push(dep.to_string());
        self
    }
}

/// Workflow DAG
struct Workflow {
    tasks: HashMap<String, Task>,
    execution_order: Vec<String>,
}

impl Workflow {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            execution_order: Vec::new(),
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id.clone(), task);
    }

    /// Topological sort for execution order
    fn compute_execution_order(&mut self) -> Result<(), String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

        // Initialize
        for id in self.tasks.keys() {
            in_degree.insert(id.clone(), 0);
            dependents.insert(id.clone(), Vec::new());
        }

        // Count in-degrees and build dependents map
        for (id, task) in &self.tasks {
            for dep in &task.dependencies {
                if !self.tasks.contains_key(dep) {
                    return Err(format!("Unknown dependency: {}", dep));
                }
                *in_degree.get_mut(id).unwrap() += 1;
                dependents.get_mut(dep).unwrap().push(id.clone());
            }
        }

        // Find initial tasks (no dependencies)
        let initial: Vec<String> = {
            let mut v: Vec<_> = in_degree
                .iter()
                .filter(|(_, &deg)| deg == 0)
                .map(|(id, _)| id.clone())
                .collect();
            v.sort(); // Sort for determinism
            v
        };

        let mut queue: VecDeque<String> = initial.into_iter().collect();

        let mut order = Vec::new();

        while let Some(id) = queue.pop_front() {
            order.push(id.clone());

            // Sort dependents for determinism
            let mut deps = dependents.get(&id).cloned().unwrap_or_default();
            deps.sort();

            for dep_id in deps {
                let deg = in_degree.get_mut(&dep_id).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(dep_id);
                }
            }
        }

        if order.len() != self.tasks.len() {
            return Err("Cycle detected in workflow".to_string());
        }

        self.execution_order = order;
        Ok(())
    }

    fn execute(&mut self) -> Vec<String> {
        let mut executed = Vec::new();

        for id in &self.execution_order {
            if let Some(task) = self.tasks.get_mut(id) {
                task.status = TaskStatus::Running;
                // Simulate task execution
                task.status = TaskStatus::Completed;
                executed.push(id.clone());
            }
        }

        executed
    }

    fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

/// Demonstrate basic workflow
fn basic_demo() {
    println!("🎼 Basic Workflow");
    println!();

    let mut workflow = Workflow::new();

    workflow.add_task(Task::new("load_data"));
    workflow.add_task(Task::new("preprocess").depends_on("load_data"));
    workflow.add_task(Task::new("train_model").depends_on("preprocess"));
    workflow.add_task(Task::new("evaluate").depends_on("train_model"));
    workflow.add_task(Task::new("deploy").depends_on("evaluate"));

    workflow.compute_execution_order().unwrap();

    println!("   Task count: {}", workflow.task_count());
    println!("   Execution order: {:?}", workflow.execution_order);
    println!();

    let executed = workflow.execute();
    println!("   Executed: {:?}", executed);
    println!();
}

/// Demonstrate parallel tasks
fn parallel_demo() {
    println!("⚡ Parallel Task Discovery");
    println!();

    let mut workflow = Workflow::new();

    // Diamond pattern
    workflow.add_task(Task::new("start"));
    workflow.add_task(Task::new("branch_a").depends_on("start"));
    workflow.add_task(Task::new("branch_b").depends_on("start"));
    workflow.add_task(
        Task::new("merge")
            .depends_on("branch_a")
            .depends_on("branch_b"),
    );

    workflow.compute_execution_order().unwrap();

    println!("   Diamond pattern workflow:");
    println!("        start");
    println!("       /     \\");
    println!("   branch_a  branch_b");
    println!("       \\     /");
    println!("        merge");
    println!();
    println!("   Execution order: {:?}", workflow.execution_order);
    println!();
}

/// Demonstrate determinism
fn determinism_demo() {
    println!("🔁 Workflow Determinism");
    println!();

    let mut results = Vec::new();

    for run in 1..=5 {
        let mut workflow = Workflow::new();

        workflow.add_task(Task::new("a"));
        workflow.add_task(Task::new("b"));
        workflow.add_task(Task::new("c").depends_on("a").depends_on("b"));
        workflow.add_task(Task::new("d").depends_on("c"));

        workflow.compute_execution_order().unwrap();
        let order = workflow.execution_order.clone();
        println!("   Run {}: {:?}", run, order);
        results.push(order);
    }

    let first = &results[0];
    let all_identical = results.iter().all(|r| r == first);

    println!();
    if all_identical {
        println!("   ✅ DETERMINISTIC: All workflow executions identical");
    } else {
        println!("   ❌ Non-deterministic: Results varied!");
    }
    println!();
}

/// Demonstrate cycle detection
fn cycle_detection_demo() {
    println!("🔄 Cycle Detection");
    println!();

    let mut workflow = Workflow::new();

    // Create a cycle: a -> b -> c -> a
    workflow.add_task(Task::new("a").depends_on("c"));
    workflow.add_task(Task::new("b").depends_on("a"));
    workflow.add_task(Task::new("c").depends_on("b"));

    match workflow.compute_execution_order() {
        Ok(_) => println!("   ❌ Should have detected cycle!"),
        Err(e) => println!("   ✅ Cycle detected: {}", e),
    }
    println!();
}

/// EU AI Act compliance
fn eu_compliance() {
    println!("🇪🇺 EU AI Act Compliance");
    println!();

    println!("   Article 10 (Data Governance):");
    println!("   ├─ Workflow definition tracked");
    println!("   ├─ Dependencies explicit");
    println!("   └─ Execution order logged");
    println!();

    println!("   Article 13 (Transparency):");
    println!("   ├─ DAG structure visible");
    println!("   ├─ Scheduling deterministic");
    println!("   └─ Task status tracked");
    println!();

    println!("   Article 15 (Robustness):");
    println!("   ├─ Cycle detection");
    println!("   ├─ Dependency validation");
    println!("   └─ Type-safe task definitions");
    println!();
}

fn main() -> Result<()> {
    println!("🎭 Chapter 17: batuta - Workflow Orchestration");
    println!();
    println!("Deterministic, reproducible workflow execution.");
    println!();
    println!("{}", "─".repeat(70));
    println!();

    basic_demo();
    println!("{}", "─".repeat(70));
    println!();

    parallel_demo();
    println!("{}", "─".repeat(70));
    println!();

    determinism_demo();
    println!("{}", "─".repeat(70));
    println!();

    cycle_detection_demo();
    println!("{}", "─".repeat(70));
    println!();

    eu_compliance();
    println!("{}", "─".repeat(70));
    println!();

    println!("🎯 Key Takeaways:");
    println!("   1. DAG-based workflow definition");
    println!("   2. Deterministic topological sort");
    println!("   3. Cycle detection for robustness");
    println!("   4. EU AI Act compliant execution");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("test").depends_on("dep1");
        assert_eq!(task.id, "test");
        assert_eq!(task.dependencies, vec!["dep1"]);
    }

    #[test]
    fn test_workflow_creation() {
        let mut workflow = Workflow::new();
        workflow.add_task(Task::new("a"));
        workflow.add_task(Task::new("b"));
        assert_eq!(workflow.task_count(), 2);
    }

    #[test]
    fn test_linear_workflow() {
        let mut workflow = Workflow::new();
        workflow.add_task(Task::new("a"));
        workflow.add_task(Task::new("b").depends_on("a"));
        workflow.add_task(Task::new("c").depends_on("b"));

        workflow.compute_execution_order().unwrap();
        assert_eq!(workflow.execution_order, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parallel_workflow() {
        let mut workflow = Workflow::new();
        workflow.add_task(Task::new("start"));
        workflow.add_task(Task::new("a").depends_on("start"));
        workflow.add_task(Task::new("b").depends_on("start"));
        workflow.add_task(Task::new("end").depends_on("a").depends_on("b"));

        workflow.compute_execution_order().unwrap();

        // start first, end last
        assert_eq!(workflow.execution_order[0], "start");
        assert_eq!(workflow.execution_order[3], "end");
    }

    #[test]
    fn test_cycle_detection() {
        let mut workflow = Workflow::new();
        workflow.add_task(Task::new("a").depends_on("b"));
        workflow.add_task(Task::new("b").depends_on("a"));

        let result = workflow.compute_execution_order();
        assert!(result.is_err());
    }

    #[test]
    fn test_execution() {
        let mut workflow = Workflow::new();
        workflow.add_task(Task::new("a"));
        workflow.add_task(Task::new("b").depends_on("a"));

        workflow.compute_execution_order().unwrap();
        let executed = workflow.execute();

        assert_eq!(executed, vec!["a", "b"]);
    }

    #[test]
    fn test_determinism() {
        let mut results = Vec::new();
        for _ in 0..5 {
            let mut workflow = Workflow::new();
            workflow.add_task(Task::new("x"));
            workflow.add_task(Task::new("y"));
            workflow.add_task(Task::new("z").depends_on("x").depends_on("y"));

            workflow.compute_execution_order().unwrap();
            results.push(workflow.execution_order.clone());
        }

        let first = &results[0];
        assert!(
            results.iter().all(|r| r == first),
            "Workflow ordering must be deterministic"
        );
    }
}

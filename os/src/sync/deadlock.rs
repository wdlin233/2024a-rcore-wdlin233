//! Deadlock avoidance algorithms
use alloc::collections::btree_map::BTreeMap;
use crate::task::current_task;

use super::UPSafeCell;

type ResourceIdentifier = usize;
type NumberOfResource = usize;
type TaskIdentifier = usize;

static DEADLOCK_DETECT_ENABLED: UPSafeCell<BTreeMap<usize, BankersAlgorithm>> = unsafe {UPSafeCell::new(BTreeMap::new())};

/// enable deadlock detection
pub fn enable() {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    trace!("Enable deadlock detect.");
    let old = DEADLOCK_DETECT_ENABLED
        .exclusive_access()
        .insert(pid, BankersAlgorithm::default());
    assert!(old.is_none());
}


/// disable deadlock detection
pub fn disable(pid: usize) {
    DEADLOCK_DETECT_ENABLED.exclusive_access().remove(&pid);
}

/// add mutually exclusive resources
pub fn add_resource(resource: ResourceIdentifier, number: NumberOfResource) {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    if let Some(algo) = DEADLOCK_DETECT_ENABLED.exclusive_access().get_mut(&pid) {
        algo.add_resource(resource, number);
    }
}

/// request sources
pub fn request(task: TaskIdentifier, resource: ResourceIdentifier, number: NumberOfResource) -> Option<RequestResult> {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    if let Some(algo) = DEADLOCK_DETECT_ENABLED.exclusive_access().get_mut(&pid) {
        Some(algo.request(task, resource, number))
    } else {
        None
    }
}

/// alloc sources
pub fn acquire(task: TaskIdentifier, resource: ResourceIdentifier, number: NumberOfResource) {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    if let Some(algo) = DEADLOCK_DETECT_ENABLED.exclusive_access().get_mut(&pid) {
        algo.alloc(task, resource, number)
    }
}

/// release sources
pub fn release(task: TaskIdentifier, resource: ResourceIdentifier, number: NumberOfResource) {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    if let Some(algo) = DEADLOCK_DETECT_ENABLED.exclusive_access().get_mut(&pid) {
        algo.dealloc(task, resource, number)
    }
}

/// record
pub fn record(task: TaskIdentifier, resource: ResourceIdentifier, number: NumberOfResource) {
    let pid = current_task().unwrap().process.upgrade().unwrap().getpid();
    if let Some(algo) = DEADLOCK_DETECT_ENABLED.exclusive_access().get_mut(&pid) {
        algo.record(task, resource, number)
    }
}


#[derive(Debug, Default)]
struct TaskResourcesState {
    // max: NumberOfResource,
    allocation: NumberOfResource,
    need: NumberOfResource,
}

/// Implementation of banker's algorithm
#[derive(Debug, Default)]
pub struct BankersAlgorithm {
    /// Available map, (Resource) = avaibable number
    available: BTreeMap<ResourceIdentifier, NumberOfResource>,
    /// (Task, Resource) = ResourceState {allo, need}
    task_state: BTreeMap<TaskIdentifier, BTreeMap<ResourceIdentifier, TaskResourcesState>>,
}

/// Banker's Algorithm Request Result
#[derive(Debug, PartialEq)]
pub enum RequestResult {
    /// Potential deadlock
    Error,
    /// Need wait
    Wait,
    /// Allocate resources
    Success,
}

impl BankersAlgorithm {
    pub fn add_resource(&mut self, resource: ResourceIdentifier, number: NumberOfResource) {
        *self.available.entry(resource).or_default() += number;
    }

    pub fn record(&mut self, task: TaskIdentifier, resource: ResourceIdentifier, number: NumberOfResource) {
        self.task_state.entry(task).or_default()
            .entry(resource).or_default()
            .need += number;
    }

    /// Handle request
    pub fn request(
        &mut self,
        task: TaskIdentifier,
        resource: ResourceIdentifier,
        number: NumberOfResource,
    ) -> RequestResult {
        self.record(task, resource, number);
        if !self.security_check() {
            return RequestResult::Error;
        }
        RequestResult::Success
    }

    fn alloc(
        &mut self,
        task: TaskIdentifier,
        resource: ResourceIdentifier,
        request: NumberOfResource,
    ) {
        let available = self.available.get_mut(&resource).unwrap();
        let task = self
            .task_state
            .get_mut(&task)
            .unwrap()
            .get_mut(&resource)
            .unwrap();
        // Available[j] = Available[j] - Request[i,j];
        *available -= request;
        // Allocation[i,j] = Allocation[i,j] + Request[i,j];
        task.allocation += request;
        // Need[i,j] = Need[i,j] - Request[i,j];
        task.need -= request;
    }

    fn dealloc(
        &mut self,
        task: TaskIdentifier,
        resource: ResourceIdentifier,
        request: NumberOfResource,
    ) {
        let available = self.available.get_mut(&resource).unwrap();
        let task = self
            .task_state
            .get_mut(&task)
            .unwrap()
            .get_mut(&resource)
            .unwrap();
        *available += request;
        task.allocation -= request;
        // it is unnecessary
        // task.need += request;
    }

    fn security_check(&self) -> bool {
        // 1. 设置两个向量:
        //   工作向量Work，表示操作系统可提供给线程继续运行所需的各类资源数目，它含有m个元素，初始时，Work = Available
        let mut work = self.available.clone();

        //   结束向量Finish，表示系统是否有足够的资源分配给线程，使之运行完成。初始时 Finish[0..n-1] = false，表示所有线程都没结束
        //   当有足够资源分配给线程时，设置Finish[i] = true。
        // TODO(fh): change to BTreeSet
        let mut finish = self
            .task_state
            .keys()
            .map(|&task| (task, false))
            .collect::<BTreeMap<_, _>>();

        loop {
            // 2. 从线程集合中找到一个能满足下述条件的线程
            // Finish[i] == false; Need[i,j] <= Work[j];
            if let Some((task, res_state)) = self.task_state.iter()
                .find(|(task, res_state)| {
                    !finish[task] && 
                    res_state.iter()
                    .all(|(res, state)| state.need <= work[res])
            }) {
                // 若找到，执行步骤3，否则，执行步骤4。
                // 3. 当线程thr[i]获得资源后，可顺利执行，直至完成，并释放出分配给它的资源，故应执行:
                // Work[j] = Work[j] + Allocation[i,j];
                for (res, state) in res_state {
                    *work.get_mut(res).unwrap() += state.allocation;
                }

                // Finish[i] = true;
                *finish.get_mut(task).unwrap() = true;

                // 跳转回步骤2
                continue;
            } else {
                // 4. 如果Finish[0..=n-1] 都为true，则表示系统处于安全状态；否则表示系统处于不安全状态。
                if finish.values().all(|&ok| ok) {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}
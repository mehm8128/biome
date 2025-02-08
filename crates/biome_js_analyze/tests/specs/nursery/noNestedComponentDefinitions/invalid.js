function ParentComponent() {
	function UnstableNestedFunctionComponent() {
		return <div />;
	}

	return (
		<div>
			<UnstableNestedFunctionComponent />
		</div>
	);
}

function ParentComponent() {
	function UnstableNestedFunctionComponent() {
		return React.createElement("div", null);
	}

	return React.createElement(
		"div",
		null,
		React.createElement(UnstableNestedFunctionComponent, null)
	);
}

function ParentComponent() {
	const UnstableNestedVariableComponent = () => {
		return <div />;
	};

	return (
		<div>
			<UnstableNestedVariableComponent />
		</div>
	);
}

function ParentComponent() {
	const UnstableNestedVariableComponent = () => {
		return React.createElement("div", null);
	};

	return React.createElement(
		"div",
		null,
		React.createElement(UnstableNestedVariableComponent, null)
	);
}

const ParentComponent = () => {
	function UnstableNestedFunctionComponent() {
		return <div />;
	}

	return (
		<div>
			<UnstableNestedFunctionComponent />
		</div>
	);
};

const ParentComponent = () => {
	function UnstableNestedFunctionComponent() {
		return React.createElement("div", null);
	}

	return React.createElement(
		"div",
		null,
		React.createElement(UnstableNestedFunctionComponent, null)
	);
};

export default () => {
	function UnstableNestedFunctionComponent() {
		return <div />;
	}

	return (
		<div>
			<UnstableNestedFunctionComponent />
		</div>
	);
};

export default () => {
	function UnstableNestedFunctionComponent() {
		return React.createElement("div", null);
	}

	return React.createElement(
		"div",
		null,
		React.createElement(UnstableNestedFunctionComponent, null)
	);
};

const ParentComponent = () => {
	const UnstableNestedVariableComponent = () => {
		return <div />;
	};

	return (
		<div>
			<UnstableNestedVariableComponent />
		</div>
	);
};

const ParentComponent = () => {
	const UnstableNestedVariableComponent = () => {
		return React.createElement("div", null);
	};

	return React.createElement(
		"div",
		null,
		React.createElement(UnstableNestedVariableComponent, null)
	);
};

function ParentComponent() {
	class UnstableNestedClassComponent extends React.Component {
		render() {
			return <div />;
		}
	}

	return (
		<div>
			<UnstableNestedClassComponent />
		</div>
	);
}

function ParentComponent() {
	class UnstableNestedClassComponent extends React.Component {
		render() {
			return React.createElement("div", null);
		}
	}

	return React.createElement(
		"div",
		null,
		React.createElement(UnstableNestedClassComponent, null)
	);
}

class ParentComponent extends React.Component {
	render() {
		class UnstableNestedClassComponent extends React.Component {
			render() {
				return <div />;
			}
		}

		return (
			<div>
				<UnstableNestedClassComponent />
			</div>
		);
	}
}

class ParentComponent extends React.Component {
	render() {
		class UnstableNestedClassComponent extends React.Component {
			render() {
				return React.createElement("div", null);
			}
		}

		return React.createElement(
			"div",
			null,
			React.createElement(UnstableNestedClassComponent, null)
		);
	}
}

class ParentComponent extends React.Component {
	render() {
		function UnstableNestedFunctionComponent() {
			return <div />;
		}

		return (
			<div>
				<UnstableNestedFunctionComponent />
			</div>
		);
	}
}

class ParentComponent extends React.Component {
	render() {
		function UnstableNestedClassComponent() {
			return React.createElement("div", null);
		}

		return React.createElement(
			"div",
			null,
			React.createElement(UnstableNestedClassComponent, null)
		);
	}
}

class ParentComponent extends React.Component {
	render() {
		const UnstableNestedVariableComponent = () => {
			return <div />;
		};

		return (
			<div>
				<UnstableNestedVariableComponent />
			</div>
		);
	}
}

class ParentComponent extends React.Component {
	render() {
		const UnstableNestedClassComponent = () => {
			return React.createElement("div", null);
		};

		return React.createElement(
			"div",
			null,
			React.createElement(UnstableNestedClassComponent, null)
		);
	}
}

function ParentComponent() {
	function getComponent() {
		function NestedUnstableFunctionComponent() {
			return <div />;
		}

		return <NestedUnstableFunctionComponent />;
	}

	return <div>{getComponent()}</div>;
}

function ParentComponent() {
	function getComponent() {
		function NestedUnstableFunctionComponent() {
			return React.createElement("div", null);
		}

		return React.createElement(NestedUnstableFunctionComponent, null);
	}

	return React.createElement("div", null, getComponent());
}

function ComponentWithProps(props) {
	return React.createElement("div", null);
}

function ParentComponent() {
	return React.createElement(ComponentWithProps, {
		footer: function SomeFooter() {
			return React.createElement("div", null);
		},
	});
}

function RenderPropComponent(props) {
	return props.render({});
}

function ParentComponent() {
	return React.createElement(RenderPropComponent, null, () => {
		function UnstableNestedComponent() {
			return React.createElement("div", null);
		}

		return React.createElement(
			"div",
			null,
			React.createElement(UnstableNestedComponent, null)
		);
	});
}

function ParentComponent() {
	return <ComponentForProps someMap={{ Header: () => <div /> }} />;
}

class ParentComponent extends React.Component {
	render() {
		const List = () => {
			return <ul>item</ul>;
		};

		return <List {...this.props} />;
	}
}

class ParentComponent extends React.Component {
	render() {
		const List = (props) => {
			const items = props.items.map((item) => (
				<li key={item.key}>
					<span>{item.name}</span>
				</li>
			));

			return <ul>{items}</ul>;
		};

		return <List {...this.props} />;
	}
}

function ParentComponent() {
	const UnstableNestedComponent = React.memo(() => {
		return <div />;
	});

	return (
		<div>
			<UnstableNestedComponent />
		</div>
	);
}

function ParentComponent() {
	const UnstableNestedComponent = React.memo(() =>
		React.createElement("div", null)
	);

	return React.createElement(
		"div",
		null,
		React.createElement(UnstableNestedComponent, null)
	);
}

function ParentComponent() {
	const UnstableNestedComponent = React.memo(function () {
		return <div />;
	});

	return (
		<div>
			<UnstableNestedComponent />
		</div>
	);
}

function ParentComponent() {
	const UnstableNestedComponent = React.memo(function () {
		return React.createElement("div", null);
	});

	return React.createElement(
		"div",
		null,
		React.createElement(UnstableNestedComponent, null)
	);
}

function ParentComponent() {
	const MemoizedNestedComponent = React.useCallback(() => <div />, []);

	return (
		<div>
			<MemoizedNestedComponent />
		</div>
	);
}

function ParentComponent() {
	const MemoizedNestedComponent = React.useCallback(
		() => React.createElement("div", null),
		[]
	);

	return React.createElement(
		"div",
		null,
		React.createElement(MemoizedNestedComponent, null)
	);
}

function ParentComponent() {
	const MemoizedNestedFunctionComponent = React.useCallback(function () {
		return <div />;
	}, []);

	return (
		<div>
			<MemoizedNestedFunctionComponent />
		</div>
	);
}

function ParentComponent() {
	const MemoizedNestedFunctionComponent = React.useCallback(function () {
		return React.createElement("div", null);
	}, []);

	return React.createElement(
		"div",
		null,
		React.createElement(MemoizedNestedFunctionComponent, null)
	);
}

function ParentComponent() {
	return <SomeComponent components={{ Header: () => <div /> }} />;
}

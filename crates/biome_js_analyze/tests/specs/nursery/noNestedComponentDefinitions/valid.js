function ParentComponent() {
	return (
		<div>
			<OutsideDefinedFunctionComponent />
		</div>
	);
}

function ParentComponent() {
	return React.createElement(
		"div",
		null,
		React.createElement(OutsideDefinedFunctionComponent, null)
	);
}

function ParentComponent() {
	return (
		<SomeComponent footer={<OutsideDefinedComponent />} header={<div />} />
	);
}

function ParentComponent() {
	return React.createElement(SomeComponent, {
		footer: React.createElement(OutsideDefinedComponent, null),
		header: React.createElement("div", null),
	});
}

function ParentComponent(props) {
	// Should not interfere handler declarations
	function onClick(event) {
		props.onClick(event.target.value);
	}

	const onKeyPress = () => null;

	function getOnHover() {
		return function onHover(event) {
			props.onHover(event.target);
		};
	}

	return (
		<div>
			<button
				onClick={onClick}
				onKeyPress={onKeyPress}
				onHover={getOnHover()}
				// These should not be considered as components
				maybeComponentOrHandlerNull={() => null}
				maybeComponentOrHandlerUndefined={() => undefined}
				maybeComponentOrHandlerBlank={() => ""}
				maybeComponentOrHandlerString={() => "hello-world"}
				maybeComponentOrHandlerNumber={() => 42}
				maybeComponentOrHandlerArray={() => []}
				maybeComponentOrHandlerObject={() => {}}
			/>
		</div>
	);
}

function ParentComponent() {
	function getComponent() {
		return <div />;
	}

	return <div>{getComponent()}</div>;
}

function ParentComponent() {
	function getComponent() {
		return React.createElement("div", null);
	}

	return React.createElement("div", null, getComponent());
}

function ParentComponent() {
	return (
		<ComplexRenderPropComponent
			listRenderer={data.map((items, index) => (
				<ul>
					{items[index].map((item) => (
						<li>{item}</li>
					))}
				</ul>
			))}
		/>
	);
}

function ParentComponent() {
	return React.createElement(RenderPropComponent, null, () =>
		React.createElement("div", null)
	);
}

function ParentComponent(props) {
	return (
		<ul>
			{props.items.map((item) => (
				<li key={item.id}>{item.name}</li>
			))}
		</ul>
	);
}

function ParentComponent(props) {
	return (
		<List
			items={props.items.map((item) => {
				return <li key={item.id}>{item.name}</li>;
			})}
		/>
	);
}

function ParentComponent(props) {
	return React.createElement(
		"ul",
		null,
		props.items.map(() =>
			React.createElement("li", { key: item.id }, item.name)
		)
	);
}

function ParentComponent(props) {
	return (
		<ul>
			{props.items.map(function Item(item) {
				return <li key={item.id}>{item.name}</li>;
			})}
		</ul>
	);
}

function ParentComponent(props) {
	return React.createElement(
		"ul",
		null,
		props.items.map(function Item() {
			return React.createElement("li", { key: item.id }, item.name);
		})
	);
}

function createTestComponent(props) {
	return <div />;
}

function createTestComponent(props) {
	return React.createElement("div", null);
}

function ParentComponent() {
	return (
		<SomeComponent>
			{thing.match({
				renderLoading: () => <div />,
				renderSuccess: () => <div />,
				renderFailure: () => <div />,
			})}
		</SomeComponent>
	);
}

function ParentComponent() {
	const thingElement = thing.match({
		renderLoading: () => <div />,
		renderSuccess: () => <div />,
		renderFailure: () => <div />,
	});
	return <SomeComponent>{thingElement}</SomeComponent>;
}

function ParentComponent() {
	return <ComponentForProps renderFooter={() => <div />} />;
}

function ParentComponent() {
	return React.createElement(ComponentForProps, {
		renderFooter: () => React.createElement("div", null),
	});
}

function ParentComponent() {
	useEffect(() => {
		return () => null;
	});

	return <div />;
}

function ParentComponent() {
	return (
		<SomeComponent
			renderMenu={() => (
				<RenderPropComponent>
					{items.map((item) => (
						<li key={item}>{item}</li>
					))}
				</RenderPropComponent>
			)}
		/>
	);
}

const ParentComponent = () => (
	<SomeComponent
		components={[
			<ul>
				{list.map((item) => (
					<li key={item}>{item}</li>
				))}
			</ul>,
		]}
	/>
);

function ParentComponent() {
	const rows = [
		{
			name: "A",
			render: (props) => <Row {...props} />,
		},
	];

	return <Table rows={rows} />;
}

function ParentComponent() {
	return <SomeComponent renderers={{ notComponent: () => null }} />;
}

const ParentComponent = createReactClass({
	displayName: "ParentComponent",
	statics: {
		getSnapshotBeforeUpdate: function () {
			return null;
		},
	},
	render() {
		return <div />;
	},
});

function ParentComponent() {
	const _renderHeader = () => <div />;
	return <div>{_renderHeader()}</div>;
}

const testCases = {
	basic: {
		render() {
			const Component = () => <div />;
			return <div />;
		},
	},
};

function ComponentWithProps(props) {
	return <div />;
}

function ParentComponent() {
	return (
		<ComponentWithProps
			footer={function SomeFooter() {
				return <div />;
			}}
		/>
	);
}

function ComponentWithProps(props) {
	return <div />;
}

function ParentComponent() {
	return <ComponentWithProps footer={() => <div />} />;
}

function ComponentForProps(props) {
	return <div />;
}

function ParentComponent() {
	return <ComponentForProps notPrefixedWithRender={() => <div />} />;
}

/** @public */
export class ErrorBoundary {
	static getDerivedStateFromError(error) {
		return { error };
	}

	state = initialState;

	componentDidCatch(error) {
		this.props.onError?.(error);
	}

	render() {
		const { error } = this.state;

		if (error !== null) {
			const { fallback: Fallback } = this.props;
			return <Fallback error={error} />;
		}

		return this.props.children;
	}
}

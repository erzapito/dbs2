import { shallowMount } from '@vue/test-utils';
import Paginator from '@/components/paginator.vue';

describe('Paginator.vue', () => {
    it('renders page 1', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 1,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.find('.first').exists()).toBe(false);
        expect(wrapper.find('.separator.left').exists()).toBe(false);
        expect(wrapper.find('.previous').exists()).toBe(false);
        expect(wrapper.find('.current').text()).toBe("1");
        expect(wrapper.find('.next').text()).toBe("2");
        expect(wrapper.find('.separator.right').exists()).toBe(true);
        expect(wrapper.find('.last').text()).toBe("10");
    });
    it('renders page 2', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 2,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.find('.first').text()).toBe("1");
        expect(wrapper.find('.separator.left').exists()).toBe(false);
        expect(wrapper.find('.previous').exists()).toBe(false);
        expect(wrapper.find('.current').text()).toBe("2");
        expect(wrapper.find('.next').text()).toBe("3");
        expect(wrapper.find('.separator.right').exists()).toBe(true);
        expect(wrapper.find('.last').text()).toBe("10");
    });
    it('renders page 3', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 3,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.find('.first').text()).toBe("1");
        expect(wrapper.find('.separator.left').exists()).toBe(false);
        expect(wrapper.find('.previous').text()).toBe("2");
        expect(wrapper.find('.current').text()).toBe("3");
        expect(wrapper.find('.next').text()).toBe("4");
        expect(wrapper.find('.separator.right').exists()).toBe(true);
        expect(wrapper.find('.last').text()).toBe("10");
    });
    it('renders page 4', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 4,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.find('.first').text()).toBe("1");
        expect(wrapper.find('.separator.left').exists()).toBe(true);
        expect(wrapper.find('.previous').text()).toBe("3");
        expect(wrapper.find('.current').text()).toBe("4");
        expect(wrapper.find('.next').text()).toBe("5");
        expect(wrapper.find('.separator.right').exists()).toBe(true);
        expect(wrapper.find('.last').text()).toBe("10");
    });
    it('renders page 8', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 8,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.find('.first').text()).toBe("1");
        expect(wrapper.find('.separator.left').exists()).toBe(true);
        expect(wrapper.find('.previous').text()).toBe("7");
        expect(wrapper.find('.current').text()).toBe("8");
        expect(wrapper.find('.next').text()).toBe("9");
        expect(wrapper.find('.separator.right').exists()).toBe(false);
        expect(wrapper.find('.last').text()).toBe("10");
    });
    it('renders page 9', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 9,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.find('.first').text()).toBe("1");
        expect(wrapper.find('.separator.left').exists()).toBe(true);
        expect(wrapper.find('.previous').text()).toBe("8");
        expect(wrapper.find('.current').text()).toBe("9");
        expect(wrapper.find('.next').exists()).toBe(false);
        expect(wrapper.find('.separator.right').exists()).toBe(false);
        expect(wrapper.find('.last').text()).toBe("10");
    });
    it('renders page 10', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 10,
                pageSize : 10,
                totalItems : 100,
            },
        });
        expect(wrapper.find('.first').text()).toBe("1");
        expect(wrapper.find('.separator.left').exists()).toBe(true);
        expect(wrapper.find('.previous').text()).toBe("9");
        expect(wrapper.find('.current').text()).toBe("10");
        expect(wrapper.find('.next').exists()).toBe(false);
        expect(wrapper.find('.separator.right').exists()).toBe(false);
        expect(wrapper.find('.last').exists()).toBe(false);
    });

    it('renders 0 items', () => {
        const wrapper = shallowMount(Paginator, {
            propsData: {
                page : 10,
                pageSize : 10,
                totalItems : 0,
            },
        });
        expect(wrapper.find('.first').exists()).toBe(false);
        expect(wrapper.find('.separator.left').exists()).toBe(false);
        expect(wrapper.find('.previous').exists()).toBe(false);
        expect(wrapper.find('.current').exists()).toBe(false);
        expect(wrapper.find('.next').exists()).toBe(false);
        expect(wrapper.find('.separator.right').exists()).toBe(false);
        expect(wrapper.find('.last').exists()).toBe(false);
    });

});
